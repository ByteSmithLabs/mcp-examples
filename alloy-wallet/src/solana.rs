use anyhow::Error;
use ic_cdk::management_canister::{
    schnorr_public_key, SchnorrAlgorithm, SchnorrKeyId, SchnorrPublicKeyArgs,
};
use solana_pubkey::Pubkey;

const KEY_NAME: &str = "dfx_test_key";

pub async fn get_address() -> Result<String, Error> {
    Ok(bs58::encode(Pubkey::try_from(public_key().await?.as_slice())?).into_string())
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
