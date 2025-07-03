#![allow(deprecated)]
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize, Debug)]
pub enum EthSepoliaService {
    Alchemy,
    BlockPi,
    PublicNode,
    Ankr,
    Sepolia,
}
#[derive(CandidType, Deserialize, Debug)]
pub enum L2MainnetService {
    Alchemy,
    Llama,
    BlockPi,
    PublicNode,
    Ankr,
}
pub type ChainId = u64;
#[derive(CandidType, Deserialize, Debug)]
pub struct HttpHeader {
    pub value: String,
    pub name: String,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct RpcApi {
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
}
#[derive(CandidType, Deserialize, Debug)]
pub enum EthMainnetService {
    Alchemy,
    Llama,
    BlockPi,
    Cloudflare,
    PublicNode,
    Ankr,
}
#[derive(CandidType, Deserialize)]
pub enum RpcServices {
    EthSepolia(Option<Vec<EthSepoliaService>>),
    BaseMainnet(Option<Vec<L2MainnetService>>),
    Custom {
        #[serde(rename = "chainId")]
        chain_id: ChainId,
        services: Vec<RpcApi>,
    },
    OptimismMainnet(Option<Vec<L2MainnetService>>),
    ArbitrumOne(Option<Vec<L2MainnetService>>),
    EthMainnet(Option<Vec<EthMainnetService>>),
}
#[derive(CandidType, Deserialize)]
pub enum ConsensusStrategy {
    Equality,
    Threshold { min: u8, total: Option<u8> },
}
#[derive(CandidType, Deserialize)]
pub struct RpcConfig {
    #[serde(rename = "responseConsensus")]
    pub response_consensus: Option<ConsensusStrategy>,
    #[serde(rename = "responseSizeEstimate")]
    pub response_size_estimate: Option<u64>,
}
#[derive(CandidType, Deserialize)]
pub enum BlockTag {
    Earliest,
    Safe,
    Finalized,
    Latest,
    Number(candid::Nat),
    Pending,
}
#[derive(CandidType, Deserialize)]
pub struct GetTransactionCountArgs {
    pub address: String,
    pub block: BlockTag,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}
#[derive(CandidType, Deserialize, Debug)]
pub enum ProviderError {
    TooFewCycles {
        expected: candid::Nat,
        received: candid::Nat,
    },
    InvalidRpcConfig(String),
    MissingRequiredProvider,
    ProviderNotFound,
    NoPermission,
}
#[derive(CandidType, Deserialize, Debug)]
pub enum ValidationError {
    Custom(String),
    InvalidHex(String),
}
#[derive(CandidType, Deserialize, Debug)]
pub enum RejectionCode {
    NoError,
    CanisterError,
    SysTransient,
    DestinationInvalid,
    Unknown,
    SysFatal,
    CanisterReject,
}
#[derive(CandidType, Deserialize, Debug)]
pub enum HttpOutcallError {
    IcError {
        code: RejectionCode,
        message: String,
    },
    InvalidHttpJsonRpcResponse {
        status: u16,
        body: String,
        #[serde(rename = "parsingError")]
        parsing_error: Option<String>,
    },
}

#[derive(CandidType, Deserialize, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum RpcError {
    JsonRpcError(JsonRpcError),
    ProviderError(ProviderError),
    ValidationError(ValidationError),
    HttpOutcallError(HttpOutcallError),
}
pub type GetTransactionCountResult = std::result::Result<candid::Nat, RpcError>;
pub type ProviderId = u64;
#[derive(CandidType, Deserialize, Debug)]
pub enum RpcService {
    EthSepolia(EthSepoliaService),
    BaseMainnet(L2MainnetService),
    Custom(RpcApi),
    OptimismMainnet(L2MainnetService),
    ArbitrumOne(L2MainnetService),
    EthMainnet(EthMainnetService),
    Provider(ProviderId),
}
#[derive(CandidType, Deserialize)]
pub enum MultiGetTransactionCountResult {
    Consistent(GetTransactionCountResult),
    Inconsistent(Vec<(RpcService, GetTransactionCountResult)>),
}
#[derive(CandidType, Deserialize)]
pub enum SendRawTransactionStatus {
    Ok(Option<String>),
    NonceTooLow,
    NonceTooHigh,
    InsufficientFunds,
}
pub type SendRawTransactionResult = std::result::Result<SendRawTransactionStatus, RpcError>;
#[derive(CandidType, Deserialize)]
pub enum MultiSendRawTransactionResult {
    Consistent(SendRawTransactionResult),
    Inconsistent(Vec<(RpcService, SendRawTransactionResult)>),
}
pub type RequestResult = std::result::Result<String, RpcError>;

pub struct Service(pub Principal);
impl Service {
    pub async fn eth_get_transaction_count(
        &self,
        arg0: &RpcServices,
        arg1: &Option<RpcConfig>,
        arg2: &GetTransactionCountArgs,
        arg3: u128,
    ) -> Result<(MultiGetTransactionCountResult,)> {
        ic_cdk::api::call::call_with_payment128(
            self.0,
            "eth_getTransactionCount",
            (arg0, arg1, arg2),
            arg3,
        )
        .await
    }
    pub async fn eth_send_raw_transaction(
        &self,
        arg0: &RpcServices,
        arg1: &Option<RpcConfig>,
        arg2: &String,
        arg3: u128,
    ) -> Result<(MultiSendRawTransactionResult,)> {
        ic_cdk::api::call::call_with_payment128(
            self.0,
            "eth_sendRawTransaction",
            (arg0, arg1, arg2),
            arg3,
        )
        .await
    }
    pub async fn request(
        &self,
        arg0: &RpcService,
        arg1: &String,
        arg2: &u64,
        arg3: u128,
    ) -> Result<(RequestResult,)> {
        ic_cdk::api::call::call_with_payment128(self.0, "request", (arg0, arg1, arg2), arg3).await
    }
}
