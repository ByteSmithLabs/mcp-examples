use anyhow::{anyhow, Error};
use ic_cdk::management_canister::{
    schnorr_public_key, SchnorrAlgorithm, SchnorrKeyId, SchnorrPublicKeyArgs,
};
use solana_pubkey::Pubkey;

mod sol_rpc;
use candid::Principal;
use sol_rpc::{
    CommitmentLevel, GetBalanceParams, GetBalanceResult, MultiGetBalanceResult, RpcSources,
    Service, SolanaCluster,
};

const KEY_NAME: &str = "dfx_test_key";
const RPC_SOURCES: RpcSources = RpcSources::Default(SolanaCluster::Devnet);

pub async fn get_address() -> Result<String, Error> {
    Ok(bs58::encode(Pubkey::try_from(public_key().await?.as_slice())?).into_string())
}

pub async fn get_balance(address: String) -> Result<u64, Error> {
    let service = Service(Principal::from_text("tghme-zyaaa-aaaar-qarca-cai").unwrap());

    let result = service
        .get_balance(
            RPC_SOURCES,
            None,
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
            name: KEY_NAME.to_string(),
        },
    })
    .await?
    .public_key)
}
