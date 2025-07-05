#![allow(non_snake_case)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::large_enum_variant)]
#![allow(deprecated)]
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;

pub type Regex = String;
#[derive(CandidType, Deserialize)]
pub enum LogFilter {
    ShowAll,
    HideAll,
    ShowPattern(Regex),
    HidePattern(Regex),
}

#[derive(CandidType, Deserialize)]
pub enum Mode {
    Demo,
    Normal,
}

#[derive(CandidType, Deserialize)]
pub struct RegexSubstitution {
    pub pattern: Regex,
    pub replacement: String,
}

#[derive(CandidType, Deserialize)]
pub struct OverrideProvider {
    pub overrideUrl: Option<RegexSubstitution>,
}

pub type NumSubnetNodes = u32;
#[derive(CandidType, Deserialize)]
pub struct InstallArgs {
    pub logFilter: Option<LogFilter>,
    pub manageApiKeys: Option<Vec<Principal>>,
    pub mode: Option<Mode>,
    pub overrideProvider: Option<OverrideProvider>,
    pub numSubnetNodes: Option<NumSubnetNodes>,
}

#[derive(CandidType, Deserialize)]
pub enum SolanaCluster {
    Mainnet,
    Testnet,
    Devnet,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct HttpHeader {
    pub value: String,
    pub name: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct RpcEndpoint {
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum SupportedProvider {
    AnkrMainnet,
    AlchemyDevnet,
    DrpcMainnet,
    ChainstackDevnet,
    AlchemyMainnet,
    HeliusDevnet,
    AnkrDevnet,
    DrpcDevnet,
    ChainstackMainnet,
    PublicNodeMainnet,
    HeliusMainnet,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RpcSource {
    Custom(RpcEndpoint),
    Supported(SupportedProvider),
}

#[derive(CandidType, Deserialize)]
pub enum RpcSources {
    Default(SolanaCluster),
    Custom(Vec<RpcSource>),
}

#[derive(CandidType, Deserialize)]
pub enum ConsensusStrategy {
    Equality,
    Threshold { min: u8, total: Option<u8> },
}

#[derive(CandidType, Deserialize)]
pub struct RpcConfig {
    pub responseConsensus: Option<ConsensusStrategy>,
    pub responseSizeEstimate: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum GetAccountInfoEncoding {
    #[serde(rename = "base64+zstd")]
    _671227429_,
    #[serde(rename = "jsonParsed")]
    JsonParsed,
    #[serde(rename = "base58")]
    Base58,
    #[serde(rename = "base64")]
    Base64,
}

pub type Pubkey = String;
#[derive(CandidType, Deserialize)]
pub struct DataSlice {
    pub offset: u32,
    pub length: u32,
}

pub type Slot = u64;
#[derive(CandidType, Deserialize)]
pub enum CommitmentLevel {
    #[serde(rename = "finalized")]
    Finalized,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "processed")]
    Processed,
}

#[derive(CandidType, Deserialize)]
pub struct GetAccountInfoParams {
    pub encoding: Option<GetAccountInfoEncoding>,
    pub pubkey: Pubkey,
    pub dataSlice: Option<DataSlice>,
    pub minContextSlot: Option<Slot>,
    pub commitment: Option<CommitmentLevel>,
}

#[derive(CandidType, Deserialize)]
pub struct ParsedAccount {
    pub space: u64,
    pub parsed: String,
    pub program: Pubkey,
}

#[derive(CandidType, Deserialize)]
pub enum AccountEncoding {
    #[serde(rename = "base64+zstd")]
    _671227429_,
    #[serde(rename = "jsonParsed")]
    JsonParsed,
    #[serde(rename = "base58")]
    Base58,
    #[serde(rename = "base64")]
    Base64,
    #[serde(rename = "binary")]
    Binary,
}

#[derive(CandidType, Deserialize)]
pub enum AccountData {
    #[serde(rename = "json")]
    Json(ParsedAccount),
    #[serde(rename = "legacyBinary")]
    LegacyBinary(String),
    #[serde(rename = "binary")]
    Binary(String, AccountEncoding),
}

#[derive(CandidType, Deserialize)]
pub struct AccountInfo {
    pub executable: bool,
    pub owner: Pubkey,
    pub lamports: u64,
    pub data: AccountData,
    pub space: u64,
    pub rentEpoch: u64,
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
    UnsupportedCluster(String),
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
        parsingError: Option<String>,
    },
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RpcError {
    JsonRpcError(JsonRpcError),
    ProviderError(ProviderError),
    ValidationError(String),
    HttpOutcallError(HttpOutcallError),
}

#[derive(CandidType, Deserialize)]
pub enum GetAccountInfoResult {
    Ok(Option<AccountInfo>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetAccountInfoResult {
    Consistent(GetAccountInfoResult),
    Inconsistent(Vec<(RpcSource, GetAccountInfoResult)>),
}

#[derive(CandidType, Deserialize)]
pub enum RequestCostResult {
    Ok(candid::Nat),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub struct GetBalanceParams {
    pub pubkey: Pubkey,
    pub minContextSlot: Option<Slot>,
    pub commitment: Option<CommitmentLevel>,
}

pub type Lamport = u64;
#[derive(CandidType, Deserialize, Debug)]
pub enum GetBalanceResult {
    Ok(Lamport),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetBalanceResult {
    Consistent(GetBalanceResult),
    Inconsistent(Vec<(RpcSource, GetBalanceResult)>),
}

#[derive(CandidType, Deserialize)]
pub enum TransactionDetails {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "accounts")]
    Accounts,
    #[serde(rename = "signatures")]
    Signatures,
}

#[derive(CandidType, Deserialize)]
pub enum GetBlockParamsCommitmentInner {
    #[serde(rename = "finalized")]
    Finalized,
    #[serde(rename = "confirmed")]
    Confirmed,
}

#[derive(CandidType, Deserialize)]
pub struct GetBlockParams {
    pub maxSupportedTransactionVersion: Option<u8>,
    pub transactionDetails: Option<TransactionDetails>,
    pub slot: Slot,
    pub rewards: Option<bool>,
    pub commitment: Option<GetBlockParamsCommitmentInner>,
}

pub type Timestamp = i64;
pub type Hash = String;
pub type Signature = String;
#[derive(CandidType, Deserialize, Debug)]
pub enum RewardRewardTypeInner {
    #[serde(rename = "fee")]
    Fee,
    #[serde(rename = "staking")]
    Staking,
    #[serde(rename = "rent")]
    Rent,
    #[serde(rename = "voting")]
    Voting,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Reward {
    pub lamports: i64,
    pub postBalance: u64,
    pub commission: Option<u8>,
    pub pubkey: Pubkey,
    pub rewardType: Option<RewardRewardTypeInner>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum InstructionError {
    ModifiedProgramId,
    CallDepth,
    Immutable,
    GenericError,
    ExecutableAccountNotRentExempt,
    IncorrectAuthority,
    PrivilegeEscalation,
    ReentrancyNotAllowed,
    InvalidInstructionData,
    RentEpochModified,
    IllegalOwner,
    ComputationalBudgetExceeded,
    ExecutableDataModified,
    ExecutableLamportChange,
    UnbalancedInstruction,
    ProgramEnvironmentSetupFailure,
    IncorrectProgramId,
    UnsupportedSysvar,
    UnsupportedProgramId,
    AccountDataTooSmall,
    NotEnoughAccountKeys,
    AccountBorrowFailed,
    InvalidRealloc,
    AccountNotExecutable,
    AccountNotRentExempt,
    Custom(u32),
    AccountDataSizeChanged,
    MaxAccountsDataAllocationsExceeded,
    ExternalAccountLamportSpend,
    ExternalAccountDataModified,
    MissingAccount,
    ProgramFailedToComplete,
    MaxInstructionTraceLengthExceeded,
    InvalidAccountData,
    ProgramFailedToCompile,
    ExecutableModified,
    InvalidAccountOwner,
    MaxSeedLengthExceeded,
    AccountAlreadyInitialized,
    AccountBorrowOutstanding,
    ReadonlyDataModified,
    UninitializedAccount,
    InvalidArgument,
    BorshIoError(String),
    BuiltinProgramsMustConsumeComputeUnits,
    MissingRequiredSignature,
    DuplicateAccountOutOfSync,
    MaxAccountsExceeded,
    ArithmeticOverflow,
    InvalidError,
    InvalidSeeds,
    DuplicateAccountIndex,
    ReadonlyLamportChange,
    InsufficientFunds,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum TransactionError {
    ProgramCacheHitMaxLimit,
    InvalidAccountForFee,
    AddressLookupTableNotFound,
    MissingSignatureForFee,
    WouldExceedAccountDataBlockLimit,
    AccountInUse,
    DuplicateInstruction(u8),
    AccountNotFound,
    TooManyAccountLocks,
    InvalidAccountIndex,
    AlreadyProcessed,
    WouldExceedAccountDataTotalLimit,
    InvalidAddressLookupTableIndex,
    SanitizeFailure,
    ResanitizationNeeded,
    InvalidRentPayingAccount,
    MaxLoadedAccountsDataSizeExceeded,
    InvalidAddressLookupTableData,
    InvalidWritableAccount,
    WouldExceedMaxAccountCostLimit,
    InvalidLoadedAccountsDataSizeLimit,
    InvalidProgramForExecution,
    InstructionError(u8, InstructionError),
    InsufficientFundsForRent { account_index: u8 },
    UnsupportedVersion,
    ClusterMaintenance,
    WouldExceedMaxVoteCostLimit,
    SignatureFailure,
    ProgramAccountNotFound,
    AccountLoadedTwice,
    ProgramExecutionTemporarilyRestricted { account_index: u8 },
    AccountBorrowOutstanding,
    WouldExceedMaxBlockCostLimit,
    InvalidAddressLookupTableOwner,
    InsufficientFundsForFee,
    CallChainTooDeep,
    UnbalancedTransaction,
    CommitCancelled,
    BlockhashNotFound,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum TransactionStatusMetaStatus {
    Ok,
    Err(TransactionError),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct TokenAmount {
    pub decimals: u8,
    pub uiAmount: Option<f64>,
    pub uiAmountString: String,
    pub amount: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct TransactionTokenBalance {
    pub uiTokenAmount: TokenAmount,
    pub owner: Option<Pubkey>,
    pub accountIndex: u8,
    pub mint: String,
    pub programId: Option<Pubkey>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CompiledInstruction {
    pub data: String,
    pub accounts: serde_bytes::ByteBuf,
    pub programIdIndex: u8,
    pub stackHeight: Option<u32>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Instruction {
    #[serde(rename = "compiled")]
    Compiled(CompiledInstruction),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct InnerInstructions {
    pub instructions: Vec<Instruction>,
    pub index: u8,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct LoadedAddresses {
    pub writable: Vec<Pubkey>,
    pub readonly: Vec<Pubkey>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct TransactionStatusMetaReturnDataInner {
    pub data: String,
    pub programId: Pubkey,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct TransactionStatusMeta {
    pub fee: u64,
    pub status: TransactionStatusMetaStatus,
    pub preBalances: Vec<u64>,
    pub postTokenBalances: Option<Vec<TransactionTokenBalance>>,
    pub innerInstructions: Option<Vec<InnerInstructions>>,
    pub postBalances: Vec<u64>,
    pub loadedAddresses: Option<LoadedAddresses>,
    pub rewards: Option<Vec<Reward>>,
    pub logMessages: Option<Vec<String>>,
    pub returnData: Option<TransactionStatusMetaReturnDataInner>,
    pub preTokenBalances: Option<Vec<TransactionTokenBalance>>,
    pub computeUnitsConsumed: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum EncodedTransactionBinary1 {
    #[serde(rename = "base58")]
    Base58,
    #[serde(rename = "base64")]
    Base64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum EncodedTransaction {
    #[serde(rename = "legacyBinary")]
    LegacyBinary(String),
    #[serde(rename = "binary")]
    Binary(String, EncodedTransactionBinary1),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum EncodedTransactionWithStatusMetaVersionInner {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "number")]
    Number(u8),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct EncodedTransactionWithStatusMeta {
    pub meta: Option<TransactionStatusMeta>,
    pub transaction: EncodedTransaction,
    pub version: Option<EncodedTransactionWithStatusMetaVersionInner>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ConfirmedBlock {
    pub numRewardPartition: Option<u64>,
    pub blockTime: Option<Timestamp>,
    pub blockhash: Hash,
    pub blockHeight: Option<u64>,
    pub signatures: Option<Vec<Signature>>,
    pub rewards: Option<Vec<Reward>>,
    pub transactions: Option<Vec<EncodedTransactionWithStatusMeta>>,
    pub previousBlockhash: Hash,
    pub parentSlot: Slot,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetBlockResult {
    Ok(Option<ConfirmedBlock>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetBlockResult {
    Consistent(GetBlockResult),
    Inconsistent(Vec<(RpcSource, GetBlockResult)>),
}

#[derive(CandidType, Deserialize)]
pub enum RpcAuth {
    BearerToken { url: String },
    UrlParameter { urlPattern: String },
}

#[derive(CandidType, Deserialize)]
pub enum RpcAccess {
    Authenticated {
        publicUrl: Option<String>,
        auth: RpcAuth,
    },
    Unauthenticated {
        publicUrl: String,
    },
}

#[derive(CandidType, Deserialize)]
pub struct RpcProvider {
    pub access: RpcAccess,
    pub cluster: SolanaCluster,
}

pub type RoundingError = u64;
#[derive(CandidType, Deserialize)]
pub struct GetRecentPrioritizationFeesRpcConfig {
    pub responseConsensus: Option<ConsensusStrategy>,
    pub maxSlotRoundingError: Option<RoundingError>,
    pub responseSizeEstimate: Option<u64>,
    pub maxLength: Option<u8>,
}

pub type GetRecentPrioritizationFeesParams = Vec<Pubkey>;
pub type MicroLamport = u64;
#[derive(CandidType, Deserialize)]
pub struct PrioritizationFee {
    pub prioritizationFee: MicroLamport,
    pub slot: Slot,
}

#[derive(CandidType, Deserialize)]
pub enum GetRecentPrioritizationFeesResult {
    Ok(Vec<PrioritizationFee>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetRecentPrioritizationFeesResult {
    Consistent(GetRecentPrioritizationFeesResult),
    Inconsistent(Vec<(RpcSource, GetRecentPrioritizationFeesResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct GetSignatureStatusesParams {
    pub searchTransactionHistory: Option<bool>,
    pub signatures: Vec<Signature>,
}

#[derive(CandidType, Deserialize)]
pub enum TransactionStatusStatus {
    Ok,
    Err(TransactionError),
}

#[derive(CandidType, Deserialize)]
pub enum TransactionConfirmationStatus {
    #[serde(rename = "finalized")]
    Finalized,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "processed")]
    Processed,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionStatus {
    pub err: Option<TransactionError>,
    pub status: TransactionStatusStatus,
    pub confirmationStatus: Option<TransactionConfirmationStatus>,
    pub slot: Slot,
}

#[derive(CandidType, Deserialize)]
pub enum GetSignatureStatusesResult {
    Ok(Vec<Option<TransactionStatus>>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetSignatureStatusesResult {
    Consistent(GetSignatureStatusesResult),
    Inconsistent(Vec<(RpcSource, GetSignatureStatusesResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct GetSignaturesForAddressParams {
    pub pubkey: Pubkey,
    pub limit: Option<u32>,
    pub before: Option<Signature>,
    pub until: Option<Signature>,
    pub minContextSlot: Option<Slot>,
    pub commitment: Option<CommitmentLevel>,
}

#[derive(CandidType, Deserialize)]
pub struct ConfirmedTransactionStatusWithSignature {
    pub err: Option<TransactionError>,
    pub signature: Signature,
    pub confirmationStatus: Option<TransactionConfirmationStatus>,
    pub memo: Option<String>,
    pub slot: Slot,
    pub blockTime: Option<Timestamp>,
}

#[derive(CandidType, Deserialize)]
pub enum GetSignaturesForAddressResult {
    Ok(Vec<ConfirmedTransactionStatusWithSignature>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetSignaturesForAddressResult {
    Consistent(GetSignaturesForAddressResult),
    Inconsistent(Vec<(RpcSource, GetSignaturesForAddressResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct GetSlotRpcConfig {
    pub roundingError: Option<RoundingError>,
    pub responseConsensus: Option<ConsensusStrategy>,
    pub responseSizeEstimate: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct GetSlotParams {
    pub minContextSlot: Option<Slot>,
    pub commitment: Option<CommitmentLevel>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetSlotResult {
    Ok(Slot),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetSlotResult {
    Consistent(GetSlotResult),
    Inconsistent(Vec<(RpcSource, GetSlotResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct GetTokenAccountBalanceParams {
    pub pubkey: Pubkey,
    pub commitment: Option<CommitmentLevel>,
}

#[derive(CandidType, Deserialize)]
pub enum GetTokenAccountBalanceResult {
    Ok(TokenAmount),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetTokenAccountBalanceResult {
    Consistent(GetTokenAccountBalanceResult),
    Inconsistent(Vec<(RpcSource, GetTokenAccountBalanceResult)>),
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionParamsEncodingInner {
    #[serde(rename = "base58")]
    Base58,
    #[serde(rename = "base64")]
    Base64,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionParams {
    pub signature: Signature,
    pub maxSupportedTransactionVersion: Option<u8>,
    pub encoding: Option<GetTransactionParamsEncodingInner>,
    pub commitment: Option<CommitmentLevel>,
}

#[derive(CandidType, Deserialize)]
pub struct EncodedConfirmedTransactionWithStatusMeta {
    pub transaction: EncodedTransactionWithStatusMeta,
    pub slot: Slot,
    pub blockTime: Option<Timestamp>,
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionResult {
    Ok(Option<EncodedConfirmedTransactionWithStatusMeta>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetTransactionResult {
    Consistent(GetTransactionResult),
    Inconsistent(Vec<(RpcSource, GetTransactionResult)>),
}

#[derive(CandidType, Deserialize)]
pub enum RequestResult {
    Ok(String),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiRequestResult {
    Consistent(RequestResult),
    Inconsistent(Vec<(RpcSource, RequestResult)>),
}

#[derive(CandidType, Deserialize)]
pub enum SendTransactionEncoding {
    #[serde(rename = "base58")]
    Base58,
    #[serde(rename = "base64")]
    Base64,
}

#[derive(CandidType, Deserialize)]
pub struct SendTransactionParams {
    pub encoding: Option<SendTransactionEncoding>,
    pub preflightCommitment: Option<CommitmentLevel>,
    pub transaction: String,
    pub maxRetries: Option<u32>,
    pub minContextSlot: Option<Slot>,
    pub skipPreflight: Option<bool>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum SendTransactionResult {
    Ok(Signature),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiSendTransactionResult {
    Consistent(SendTransactionResult),
    Inconsistent(Vec<(RpcSource, SendTransactionResult)>),
}

pub struct Service(pub Principal);
impl Service {
    pub async fn get_account_info(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetAccountInfoParams,
    ) -> Result<(MultiGetAccountInfoResult,)> {
        ic_cdk::call(self.0, "getAccountInfo", (arg0, arg1, arg2)).await
    }
    pub async fn get_account_info_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetAccountInfoParams,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(self.0, "getAccountInfoCyclesCost", (arg0, arg1, arg2)).await
    }
    pub async fn get_balance(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetBalanceParams,
    ) -> Result<(MultiGetBalanceResult,)> {
        ic_cdk::call(self.0, "getBalance", (arg0, arg1, arg2)).await
    }
    pub async fn get_balance_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetBalanceParams,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(self.0, "getBalanceCyclesCost", (arg0, arg1, arg2)).await
    }
    pub async fn get_block(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetBlockParams,
    ) -> Result<(MultiGetBlockResult,)> {
        ic_cdk::call(self.0, "getBlock", (arg0, arg1, arg2)).await
    }
    pub async fn get_block_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetBlockParams,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(self.0, "getBlockCyclesCost", (arg0, arg1, arg2)).await
    }
    pub async fn get_providers(&self) -> Result<(Vec<(SupportedProvider, RpcProvider)>,)> {
        ic_cdk::call(self.0, "getProviders", ()).await
    }
    pub async fn get_recent_prioritization_fees(
        &self,
        arg0: RpcSources,
        arg1: Option<GetRecentPrioritizationFeesRpcConfig>,
        arg2: Option<GetRecentPrioritizationFeesParams>,
    ) -> Result<(MultiGetRecentPrioritizationFeesResult,)> {
        ic_cdk::call(self.0, "getRecentPrioritizationFees", (arg0, arg1, arg2)).await
    }
    pub async fn get_recent_prioritization_fees_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<GetRecentPrioritizationFeesRpcConfig>,
        arg2: Option<GetRecentPrioritizationFeesParams>,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(
            self.0,
            "getRecentPrioritizationFeesCyclesCost",
            (arg0, arg1, arg2),
        )
        .await
    }
    pub async fn get_signature_statuses(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetSignatureStatusesParams,
    ) -> Result<(MultiGetSignatureStatusesResult,)> {
        ic_cdk::call(self.0, "getSignatureStatuses", (arg0, arg1, arg2)).await
    }
    pub async fn get_signature_statuses_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetSignatureStatusesParams,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(self.0, "getSignatureStatusesCyclesCost", (arg0, arg1, arg2)).await
    }
    pub async fn get_signatures_for_address(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetSignaturesForAddressParams,
    ) -> Result<(MultiGetSignaturesForAddressResult,)> {
        ic_cdk::call(self.0, "getSignaturesForAddress", (arg0, arg1, arg2)).await
    }
    pub async fn get_signatures_for_address_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetSignaturesForAddressParams,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(
            self.0,
            "getSignaturesForAddressCyclesCost",
            (arg0, arg1, arg2),
        )
        .await
    }
    pub async fn get_slot(
        &self,
        arg0: RpcSources,
        arg1: Option<GetSlotRpcConfig>,
        arg2: Option<GetSlotParams>,
    ) -> Result<(MultiGetSlotResult,)> {
        ic_cdk::call(self.0, "getSlot", (arg0, arg1, arg2)).await
    }
    pub async fn get_slot_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<GetSlotRpcConfig>,
        arg2: Option<GetSlotParams>,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(self.0, "getSlotCyclesCost", (arg0, arg1, arg2)).await
    }
    pub async fn get_token_account_balance(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetTokenAccountBalanceParams,
    ) -> Result<(MultiGetTokenAccountBalanceResult,)> {
        ic_cdk::call(self.0, "getTokenAccountBalance", (arg0, arg1, arg2)).await
    }
    pub async fn get_token_account_balance_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetTokenAccountBalanceParams,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(
            self.0,
            "getTokenAccountBalanceCyclesCost",
            (arg0, arg1, arg2),
        )
        .await
    }
    pub async fn get_transaction(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetTransactionParams,
    ) -> Result<(MultiGetTransactionResult,)> {
        ic_cdk::call(self.0, "getTransaction", (arg0, arg1, arg2)).await
    }
    pub async fn get_transaction_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: GetTransactionParams,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(self.0, "getTransactionCyclesCost", (arg0, arg1, arg2)).await
    }
    pub async fn json_request(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: String,
    ) -> Result<(MultiRequestResult,)> {
        ic_cdk::call(self.0, "jsonRequest", (arg0, arg1, arg2)).await
    }
    pub async fn json_request_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: String,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(self.0, "jsonRequestCyclesCost", (arg0, arg1, arg2)).await
    }
    pub async fn send_transaction(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: SendTransactionParams,
    ) -> Result<(MultiSendTransactionResult,)> {
        ic_cdk::call(self.0, "sendTransaction", (arg0, arg1, arg2)).await
    }
    pub async fn send_transaction_cycles_cost(
        &self,
        arg0: RpcSources,
        arg1: Option<RpcConfig>,
        arg2: SendTransactionParams,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(self.0, "sendTransactionCyclesCost", (arg0, arg1, arg2)).await
    }
    pub async fn update_api_keys(
        &self,
        arg0: Vec<(SupportedProvider, Option<String>)>,
    ) -> Result<()> {
        ic_cdk::call(self.0, "updateApiKeys", (arg0,)).await
    }
}
