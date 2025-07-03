#![allow(deprecated)]
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult as Result;
use serde::Serialize;

#[derive(CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}
#[derive(CandidType, Deserialize)]
pub struct GetAccountTransactionsArgs {
    pub max_results: candid::Nat,
    pub start: Option<candid::Nat>,
    pub account: Account,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct Tokens {
    #[serde(rename = "e8s")]
    pub e_8_s: u64,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct TimeStamp {
    pub timestamp_nanos: u64,
}
#[derive(CandidType, Deserialize, Serialize)]
pub enum Operation {
    Approve {
        fee: Tokens,
        from: String,
        allowance: Tokens,
        expected_allowance: Option<Tokens>,
        expires_at: Option<TimeStamp>,
        spender: String,
    },
    Burn {
        from: String,
        amount: Tokens,
        spender: Option<String>,
    },
    Mint {
        to: String,
        amount: Tokens,
    },
    Transfer {
        to: String,
        fee: Tokens,
        from: String,
        amount: Tokens,
        spender: Option<String>,
    },
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct Transaction {
    pub memo: u64,
    #[serde(rename = "icrc1_memo")]
    pub icrc_1_memo: Option<serde_bytes::ByteBuf>,
    pub operation: Operation,
    pub timestamp: Option<TimeStamp>,
    pub created_at_time: Option<TimeStamp>,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct TransactionWithId {
    pub id: u64,
    pub transaction: Transaction,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct GetAccountIdentifierTransactionsResponse {
    pub balance: u64,
    pub transactions: Vec<TransactionWithId>,
    pub oldest_tx_id: Option<u64>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct GetAccountIdentifierTransactionsError {
    pub message: String,
}
pub type GetAccountIdentifierTransactionsResult = std::result::Result<
    GetAccountIdentifierTransactionsResponse,
    GetAccountIdentifierTransactionsError,
>;

pub struct Service(pub Principal);
impl Service {
    pub async fn get_account_transactions(
        &self,
        arg0: &GetAccountTransactionsArgs,
    ) -> Result<(GetAccountIdentifierTransactionsResult,)> {
        ic_cdk::call(self.0, "get_account_transactions", (arg0,)).await
    }
}
