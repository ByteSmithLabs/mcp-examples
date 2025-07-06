use anyhow::{anyhow, Error};
use ic_cdk::management_canister::{
    schnorr_public_key, sign_with_schnorr, SchnorrAlgorithm, SchnorrKeyId, SchnorrPublicKeyArgs,
    SignWithSchnorrArgs,
};
use solana_pubkey::Pubkey;

mod sol_rpc;
use crate::{read_key_name, read_mode, Mode};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use bincode::serialize;
use candid::Principal;
use sol_rpc::{
    CommitmentLevel, ConsensusStrategy, GetBalanceParams, GetBalanceResult, GetBlockParams,
    GetBlockParamsCommitmentInner, GetBlockResult, GetSlotParams, GetSlotResult, GetSlotRpcConfig,
    MultiGetBalanceResult, MultiGetBlockResult, MultiGetSlotResult, MultiSendTransactionResult,
    RpcConfig, RpcSources, SendTransactionEncoding, SendTransactionParams, SendTransactionResult,
    Service, SolanaCluster,
};
use solana_hash::Hash;
use solana_message::Message;
use solana_program::system_instruction;
use solana_signature::Signature;
use solana_transaction::Transaction;
use std::str::FromStr;

fn rpc_sources() -> RpcSources {
    match read_mode() {
        Mode::Test => RpcSources::Default(SolanaCluster::Devnet),
        Mode::Production => RpcSources::Default(SolanaCluster::Mainnet),
    }
}

pub async fn get_address() -> Result<String, Error> {
    Ok(bs58::encode(Pubkey::try_from(public_key().await?.as_slice())?).into_string())
}

pub async fn get_balance(address: String) -> Result<u64, Error> {
    let service = Service(Principal::from_text("tghme-zyaaa-aaaar-qarca-cai").unwrap());

    let result = service
        .get_balance(
            rpc_sources(),
            Some(RpcConfig {
                responseConsensus: Some(ConsensusStrategy::Equality),
                responseSizeEstimate: None,
            }),
            GetBalanceParams {
                pubkey: address,
                minContextSlot: None,
                commitment: Some(CommitmentLevel::Finalized),
            },
        )
        .await
        .map_err(|err| anyhow!(format!("{:?}", err)))?
        .0;

    match result {
        MultiGetBalanceResult::Inconsistent(results) => Err(anyhow!(format!("{:?}", results))),
        MultiGetBalanceResult::Consistent(result) => match result {
            GetBalanceResult::Err(err) => Err(anyhow!(format!("{:?}", err))),
            GetBalanceResult::Ok(lamports) => Ok(lamports),
        },
    }
}

async fn public_key() -> Result<Vec<u8>, Error> {
    Ok(schnorr_public_key(&SchnorrPublicKeyArgs {
        canister_id: None,
        derivation_path: vec![],
        key_id: SchnorrKeyId {
            algorithm: SchnorrAlgorithm::Ed25519,
            name: read_key_name(),
        },
    })
    .await?
    .public_key)
}

pub async fn transfer(address: String, amount: u64) -> Result<String, Error> {
    let service = Service(Principal::from_text("tghme-zyaaa-aaaar-qarca-cai").unwrap());

    let sender = get_address().await?;

    let instruction = system_instruction::transfer(
        &Pubkey::from_str(&sender)?,
        &Pubkey::from_str(&address)?,
        amount,
    );

    let message = Message::new_with_blockhash(
        &[instruction],
        Some(&Pubkey::from_str(&sender)?),
        &Hash::from_str(&estimate_recent_blockhash().await?)?,
    );

    let signatures = vec![sign_message(&message).await?];

    let transaction = Transaction {
        message,
        signatures,
    };

    let transaction = STANDARD.encode(serialize(&transaction)?);

    let response = service
        .send_transaction(
            rpc_sources(),
            Some(RpcConfig {
                responseConsensus: Some(ConsensusStrategy::Equality),
                responseSizeEstimate: None,
            }),
            SendTransactionParams {
                encoding: Some(SendTransactionEncoding::Base64),
                preflightCommitment: None,
                transaction,
                maxRetries: None,
                minContextSlot: None,
                skipPreflight: None,
            },
        )
        .await
        .map_err(|err| anyhow!(format!("{:?}", err)))?
        .0;

    match response {
        MultiSendTransactionResult::Inconsistent(results) => Err(anyhow!(format!("{:?}", results))),
        MultiSendTransactionResult::Consistent(result) => match result {
            SendTransactionResult::Err(err) => Err(anyhow!(format!("{:?}", err))),
            SendTransactionResult::Ok(sig) => Ok(sig),
        },
    }
}

async fn sign_message(message: &Message) -> Result<Signature, Error> {
    Signature::try_from(
        sign_with_schnorr(&SignWithSchnorrArgs {
            message: message.serialize(),
            derivation_path: vec![],
            key_id: SchnorrKeyId {
                algorithm: SchnorrAlgorithm::Ed25519,
                name: read_key_name(),
            },
            aux: None,
        })
        .await?
        .signature,
    )
    .map_err(|err| anyhow!(format!("{:?}", err)))
}

async fn get_slot() -> Result<u64, Error> {
    let service = Service(Principal::from_text("tghme-zyaaa-aaaar-qarca-cai").unwrap());

    let result = service
        .get_slot(
            rpc_sources(),
            Some(GetSlotRpcConfig {
                roundingError: None,
                responseConsensus: Some(ConsensusStrategy::Equality),
                responseSizeEstimate: None,
            }),
            Some(GetSlotParams {
                minContextSlot: None,
                commitment: Some(CommitmentLevel::Finalized),
            }),
        )
        .await
        .map_err(|err| anyhow!(format!("{:?}", err)))?
        .0;

    match result {
        MultiGetSlotResult::Inconsistent(results) => Err(anyhow!(format!("{:?}", results))),
        MultiGetSlotResult::Consistent(result) => match result {
            GetSlotResult::Err(err) => Err(anyhow!(format!("{:?}", err))),
            GetSlotResult::Ok(slot) => Ok(slot),
        },
    }
}

async fn get_blockhash(slot: u64) -> Result<String, Error> {
    let service = Service(Principal::from_text("tghme-zyaaa-aaaar-qarca-cai").unwrap());

    let result = service
        .get_block(
            rpc_sources(),
            Some(RpcConfig {
                responseConsensus: Some(ConsensusStrategy::Equality),
                responseSizeEstimate: None,
            }),
            GetBlockParams {
                maxSupportedTransactionVersion: None,
                transactionDetails: None,
                slot,
                rewards: None,
                commitment: Some(GetBlockParamsCommitmentInner::Finalized),
            },
        )
        .await
        .map_err(|err| anyhow!(format!("{:?}", err)))?
        .0;

    match result {
        MultiGetBlockResult::Inconsistent(results) => Err(anyhow!(format!("{:?}", results))),
        MultiGetBlockResult::Consistent(result) => match result {
            GetBlockResult::Err(err) => Err(anyhow!(format!("{:?}", err))),
            GetBlockResult::Ok(block) => match block {
                None => Err(anyhow!("not found block")),
                Some(block) => Ok(block.blockhash),
            },
        },
    }
}

async fn estimate_recent_blockhash() -> Result<String, Error> {
    let mut errors = Vec::with_capacity(3);
    while errors.len() < 3 {
        let slot = get_slot().await;

        if let Err(err) = slot {
            errors.push(err);
            continue;
        }

        let blockhash = get_blockhash(slot.unwrap()).await;

        if let Err(err) = blockhash {
            errors.push(err);
            continue;
        }

        return Ok(blockhash.unwrap());
    }

    Err(anyhow!(errors
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("; ")))
}
