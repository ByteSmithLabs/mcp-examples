use alloy_consensus::{SignableTransaction, TxEip1559, TxEnvelope};
use alloy_primitives::{hex, Signature, TxKind, U256};
use anyhow::{anyhow, Error};
use candid::{Nat, Principal};
use ethers_core::abi::ethereum_types::Address;
use ethers_core::k256::elliptic_curve::sec1::ToEncodedPoint;
use ethers_core::k256::PublicKey;
use ethers_core::utils::keccak256;
use ic_cdk::management_canister::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgs,
    SignWithEcdsaArgs,
};
use ic_secp256k1::RecoveryId;

mod evm_rpc;
use evm_rpc::{
    BlockTag, EthSepoliaService, GetTransactionCountArgs, GetTransactionCountResult,
    MultiGetTransactionCountResult, RpcService, RpcServices, Service,
};
use num::{BigUint, Num};
use serde_json::{from_str, Value};

const RPCSERVICE: RpcService = RpcService::EthSepolia(EthSepoliaService::PublicNode);
fn rpc_services() -> RpcServices {
    RpcServices::EthSepolia(Some(vec![EthSepoliaService::PublicNode]))
}
const DERIVATION_PATH: [[u8; 0]; 0] = [];
const KEY_NAME: &str = "dfx_test_key";
const CHAIN_ID: u64 = 11155111;

pub async fn get_address() -> Result<String, Error> {
    let key = PublicKey::from_sec1_bytes(public_key().await?.as_slice())?;
    let point = key.to_encoded_point(false);
    let point_bytes = point.as_bytes();
    assert_eq!(point_bytes[0], 0x04);

    let hash = keccak256(&point_bytes[1..]);

    Ok(ethers_core::utils::to_checksum(
        &Address::from_slice(&hash[12..32]),
        None,
    ))
}

pub async fn transfer(to: String, amount: u128) -> Result<String, Error> {
    use alloy_eips::eip2718::Encodable2718;

    let transaction = TxEip1559 {
        chain_id: CHAIN_ID,
        nonce: nat_to_u64(
            get_transaction_count(get_address().await?.clone(), BlockTag::Latest).await?,
        ),
        gas_limit: 21_000,
        max_fee_per_gas: 30_000_000_000,
        max_priority_fee_per_gas: 1_500_000_000,
        to: TxKind::Call(to.parse()?),
        value: U256::from(amount),
        access_list: Default::default(),
        input: Default::default(),
    };

    let tx_hash = transaction.signature_hash().0;

    let (raw_signature, recovery_id) = sign(tx_hash).await?;
    let signature = Signature::from_bytes_and_parity(&raw_signature, recovery_id.is_y_odd())?;
    let signed_tx = transaction.into_signed(signature);
    let raw_transaction_hash = *signed_tx.hash();
    let mut tx_bytes: Vec<u8> = vec![];
    TxEnvelope::from(signed_tx).encode_2718(&mut tx_bytes);
    let raw_transaction_hex = format!("0x{}", hex::encode(&tx_bytes));

    let _ = Service(Principal::from_text("7hfb6-caaaa-aaaar-qadga-cai").unwrap())
        .eth_send_raw_transaction(
            &rpc_services(),
            &None,
            &raw_transaction_hex.clone(),
            2_000_000_000_u128,
        )
        .await
        .map_err(|err| anyhow!(format!("{:?}", err)))?
        .0;
    Ok(raw_transaction_hash.to_string())
}

async fn get_transaction_count(address: String, block: BlockTag) -> Result<Nat, Error> {
    let result: MultiGetTransactionCountResult =
        Service(Principal::from_text("7hfb6-caaaa-aaaar-qadga-cai").unwrap())
            .eth_get_transaction_count(
                &rpc_services(),
                &None,
                &GetTransactionCountArgs { address, block },
                2_000_000_000_u128,
            )
            .await
            .map_err(|err| anyhow!(format!("{:?}", err)))?
            .0;

    match result {
        MultiGetTransactionCountResult::Inconsistent(inconsistent_results) => {
            Err(anyhow!(format!("{:?}", inconsistent_results)))
        }
        MultiGetTransactionCountResult::Consistent(result) => match result {
            GetTransactionCountResult::Ok(count) => Ok(count),
            GetTransactionCountResult::Err(err) => Err(anyhow!(format!("{:?}", err))),
        },
    }
}

async fn sign(message_hash: [u8; 32]) -> Result<([u8; 64], RecoveryId), Error> {
    let result = sign_with_ecdsa(&SignWithEcdsaArgs {
        message_hash: message_hash.to_vec(),
        derivation_path: DERIVATION_PATH.iter().map(|row| row.to_vec()).collect(),
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: KEY_NAME.to_string(),
        },
    })
    .await?;

    let signature =
        <[u8; 64]>::try_from(result.signature).map_err(|err| anyhow!(format!("{:?}", err)))?;

    Ok((
        signature,
        ic_secp256k1::PublicKey::deserialize_sec1(public_key().await?.as_slice())?
            .try_recovery_from_digest(&message_hash, &signature)
            .map_err(|err| anyhow!(format!("{:?}", err)))?,
    ))
}

pub async fn get_balance(address: String) -> Result<Nat, Error> {
    let hex_balance:Value  = from_str(&Service(Principal::from_text("7hfb6-caaaa-aaaar-qadga-cai").unwrap()).request(&RPCSERVICE, &format!(
        r#"{{ "jsonrpc": "2.0", "method": "eth_getBalance", "params": ["{address}", "latest"], "id": 1 }}"#
    ).to_string(), &500_u64, 2000000000).await.map_err(|err| anyhow!(format!("{:?}", err)))?.0.map_err(|err| anyhow!(format!("{:?}", err)))?)?;

    Ok(Nat(BigUint::from_str_radix(
        &hex_balance
            .get("result")
            .and_then(|v| v.as_str())
            .ok_or(anyhow!("RPC response doesn't has result"))?
            .to_string()[2..],
        16,
    )?))
}

async fn public_key() -> Result<Vec<u8>, Error> {
    Ok(ecdsa_public_key(&EcdsaPublicKeyArgs {
        canister_id: None,
        derivation_path: DERIVATION_PATH.iter().map(|row| row.to_vec()).collect(),
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: KEY_NAME.to_string(),
        },
    })
    .await?
    .public_key)
}

fn nat_to_u64(nat: Nat) -> u64 {
    use num_traits::cast::ToPrimitive;
    nat.0
        .to_u64()
        .unwrap_or_else(|| ic_cdk::trap(format!("Nat {nat} doesn't fit into a u64")))
}
