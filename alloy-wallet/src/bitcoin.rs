use std::str::FromStr;

use anyhow::{anyhow, Error};
use bitcoin::absolute::LockTime;
use bitcoin::consensus::serialize;
use bitcoin::hashes::Hash;
use bitcoin::script::{Builder, PushBytesBuf};
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::sighash::{EcdsaSighashType, SighashCache};
use bitcoin::transaction::Version;
use bitcoin::{
    Address, AddressType, Amount, OutPoint, PublicKey, ScriptBuf, Sequence, Transaction, TxIn,
    TxOut, Txid, Witness,
};

use ic_cdk::bitcoin_canister::{
    bitcoin_get_balance, bitcoin_get_current_fee_percentiles, bitcoin_get_utxos,
    bitcoin_send_transaction, GetBalanceRequest, GetCurrentFeePercentilesRequest, GetUtxosRequest,
    MillisatoshiPerByte, Satoshi, SendTransactionRequest, Utxo,
};
use ic_cdk::management_canister::{ecdsa_public_key, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgs};

const NETWORK: bitcoin::Network = bitcoin::Network::Regtest;
const IC_NETWORK: ic_cdk::bitcoin_canister::Network = ic_cdk::bitcoin_canister::Network::Regtest;
const KEY_NAME: &str = "dfx_test_key";

pub async fn get_p2pkh_address() -> Result<String, Error> {
    let derivation_path = DerivationPath::p2pkh(0, 0);

    let public_key = get_ecdsa_public_key(derivation_path.to_vec_u8_path()).await?;

    let public_key = PublicKey::from_slice(&public_key)?;

    Ok(Address::p2pkh(public_key, NETWORK).to_string())
}

enum Purpose {
    P2PKH,
}

impl Purpose {
    fn to_u32(&self) -> u32 {
        match self {
            Purpose::P2PKH => 44,
        }
    }
}

pub struct DerivationPath {
    purpose: Purpose,

    coin_type: u32,

    account: u32,

    change: u32,

    address_index: u32,
}

impl DerivationPath {
    fn new(purpose: Purpose, account: u32, address_index: u32) -> Self {
        Self {
            purpose,
            coin_type: 0,
            account,
            change: 0,
            address_index,
        }
    }

    pub fn p2pkh(account: u32, address_index: u32) -> Self {
        Self::new(Purpose::P2PKH, account, address_index)
    }

    const HARDENED_OFFSET: u32 = 0x8000_0000;

    pub fn to_vec_u8_path(&self) -> Vec<Vec<u8>> {
        vec![
            (self.purpose.to_u32() | Self::HARDENED_OFFSET)
                .to_be_bytes()
                .to_vec(),
            (self.coin_type | Self::HARDENED_OFFSET)
                .to_be_bytes()
                .to_vec(),
            (self.account | Self::HARDENED_OFFSET)
                .to_be_bytes()
                .to_vec(),
            self.change.to_be_bytes().to_vec(),
            self.address_index.to_be_bytes().to_vec(),
        ]
    }
}

async fn get_ecdsa_public_key(derivation_path: Vec<Vec<u8>>) -> Result<Vec<u8>, Error> {
    Ok(ecdsa_public_key(&EcdsaPublicKeyArgs {
        canister_id: None,
        derivation_path: derivation_path.clone(),
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: KEY_NAME.to_string(),
        },
    })
    .await?
    .public_key)
}

pub async fn get_balance(address: String) -> Result<u64, Error> {
    Ok(bitcoin_get_balance(&GetBalanceRequest {
        address,
        network: IC_NETWORK,
        min_confirmations: None,
    })
    .await?)
}

pub async fn send_from_p2pkh_address(
    destination_address: String,
    amount_in_satoshi: u64,
) -> Result<String, Error> {
    if amount_in_satoshi == 0 {
        return Err(anyhow!("Amount must be greater than 0"));
    }

    let dst_address = Address::from_str(&destination_address)?.require_network(NETWORK)?;

    let derivation_path = DerivationPath::p2pkh(0, 0);

    let own_public_key = get_ecdsa_public_key(derivation_path.to_vec_u8_path()).await?;

    let own_public_key = PublicKey::from_slice(&own_public_key)?;

    let own_address = Address::p2pkh(own_public_key, NETWORK);

    // assume response contains all UTXOs
    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: own_address.to_string(),
        network: IC_NETWORK,
        filter: None,
    })
    .await?
    .utxos;

    // Build the transaction.
    let fee_per_byte = get_fee_per_byte().await?;
    let transaction = build_transaction(
        &own_public_key,
        &own_address,
        &own_utxos,
        &dst_address,
        amount_in_satoshi,
        fee_per_byte,
    )
    .await;

    // Sign the transaction.
    let signed_transaction = sign_transaction(
        &own_public_key,
        &own_address,
        transaction,
        derivation_path.to_vec_u8_path(),
        sign_with_ecdsa,
    )
    .await;

    // Send the transaction to the Bitcoin API.
    bitcoin_send_transaction(&SendTransactionRequest {
        network: IC_NETWORK,
        transaction: serialize(&signed_transaction),
    })
    .await
    .unwrap();

    Ok(signed_transaction.compute_txid().to_string())
}

async fn sign_with_ecdsa(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    message_hash: Vec<u8>,
) -> Signature {
    let signature = ic_cdk::management_canister::sign_with_ecdsa(
        &ic_cdk::management_canister::SignWithEcdsaArgs {
            message_hash,
            derivation_path,
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name,
            },
        },
    )
    .await
    .unwrap()
    .signature;

    Signature::from_compact(&signature).unwrap()
}

pub async fn get_fee_per_byte() -> Result<u64, Error> {
    let fee_percentiles = bitcoin_get_current_fee_percentiles(&GetCurrentFeePercentilesRequest {
        network: IC_NETWORK,
    })
    .await?;

    if fee_percentiles.is_empty() {
        // If the percentiles list is empty, we're likely on a regtest network
        // with no standard transactions. Use a fixed fallback value.
        Ok(2000) // 2 sat/vB in millisatoshis
    } else {
        // Use the 50th percentile (median) fee for balanced confirmation time and cost.
        Ok(fee_percentiles[50])
    }
}

pub async fn build_transaction(
    own_public_key: &PublicKey,
    own_address: &Address,
    own_utxos: &[Utxo],
    dst_address: &Address,
    amount: Satoshi,
    fee_per_vbyte: MillisatoshiPerByte,
) -> Transaction {
    let mut fee = 0;
    loop {
        let (transaction, _) =
            build_transaction_with_fee(own_utxos, own_address, dst_address, amount, fee).unwrap();

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        let signed_transaction = sign_transaction(
            own_public_key,
            own_address,
            transaction.clone(),
            vec![], // mock derivation path
            mock_sign_with_ecdsa,
        )
        .await;

        let tx_vsize = signed_transaction.vsize() as u64;

        if (tx_vsize * fee_per_vbyte) / 1000 == fee {
            return transaction;
        } else {
            fee = (tx_vsize * fee_per_vbyte) / 1000;
        }
    }
}

async fn mock_sign_with_ecdsa(
    _key_name: String,
    _derivation_path: Vec<Vec<u8>>,
    _signing_data: Vec<u8>,
) -> Signature {
    let r_s = [1u8; 64];
    Signature::from_compact(&r_s).unwrap()
}

async fn sign_transaction<SignFun, Fut>(
    own_public_key: &PublicKey,
    own_address: &Address,
    mut transaction: Transaction,
    derivation_path: Vec<Vec<u8>>,
    signer: SignFun,
) -> Transaction
where
    SignFun: Fn(String, Vec<Vec<u8>>, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Signature>,
{
    assert_eq!(
        own_address.address_type(),
        Some(AddressType::P2pkh),
        "Only P2PKH addresses are supported"
    );

    let transaction_clone = transaction.clone();
    let sighash_cache = SighashCache::new(&transaction_clone);

    for (index, input) in transaction.input.iter_mut().enumerate() {
        let sighash = sighash_cache
            .legacy_signature_hash(
                index,
                &own_address.script_pubkey(),
                EcdsaSighashType::All.to_u32(),
            )
            .unwrap();

        let signature = signer(
            KEY_NAME.to_string(),
            derivation_path.clone(),
            sighash.as_byte_array().to_vec(),
        )
        .await;

        let mut signature = signature.serialize_der().to_vec();
        signature.push(EcdsaSighashType::All.to_u32() as u8);

        let sig_bytes = PushBytesBuf::try_from(signature).unwrap();
        let pubkey_bytes = PushBytesBuf::try_from(own_public_key.to_bytes()).unwrap();

        input.script_sig = Builder::new()
            .push_slice(sig_bytes)
            .push_slice(pubkey_bytes)
            .into_script();

        input.witness = Witness::new();
    }

    transaction
}

fn build_transaction_with_fee(
    own_utxos: &[Utxo],
    own_address: &Address,
    dst_address: &Address,
    amount: u64,
    fee: u64,
) -> Result<(Transaction, Vec<TxOut>), String> {
    // Define a dust threshold below which change outputs are discarded.
    const DUST_THRESHOLD: u64 = 1_000;

    // --- Input Selection ---
    // Greedily select UTXOs in reverse order (oldest last) until we cover amount + fee.
    let mut utxos_to_spend = vec![];
    let mut total_spent = 0;
    for utxo in own_utxos.iter().rev() {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo);
        if total_spent >= amount + fee {
            break;
        }
    }

    // Abort if we can't cover the payment + fee.
    if total_spent < amount + fee {
        return Err(format!(
            "Insufficient balance: {total_spent}, trying to transfer {amount} satoshi with fee {fee}"
        ));
    }

    // --- Build Inputs ---
    let inputs: Vec<TxIn> = utxos_to_spend
        .iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_raw_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: Sequence::MAX,
            witness: Witness::new(),      // Will be filled in during signing
            script_sig: ScriptBuf::new(), // Empty for SegWit or Taproot
        })
        .collect();

    // --- Create Previous Outputs ---
    // Each TxOut struct represents an output of a previous transaction that is now being spent.
    // This information is needed later when signing transactions for P2TR and P2WPKH.
    let prevouts = utxos_to_spend
        .into_iter()
        .map(|utxo| TxOut {
            value: Amount::from_sat(utxo.value),
            script_pubkey: own_address.script_pubkey(),
        })
        .collect();

    // --- Build Outputs ---
    // Primary output: send amount to destination.
    let mut outputs = vec![TxOut {
        script_pubkey: dst_address.script_pubkey(),
        value: Amount::from_sat(amount),
    }];

    // Add a change output if the remainder is above the dust threshold.
    let change = total_spent - amount - fee;
    if change >= DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: own_address.script_pubkey(),
            value: Amount::from_sat(change),
        });
    }

    // --- Assemble Transaction ---
    Ok((
        Transaction {
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO,
            version: Version::TWO,
        },
        prevouts,
    ))
}
