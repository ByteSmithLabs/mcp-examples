use ic_cdk::{init, query, update};
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use ic_rmcp::{Handler, Server};
use rmcp::{handler::server::tool::schema_for_type, model::*, Error};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{from_value, Value};
use std::cell::RefCell;

mod bitcoin;
use bitcoin::{get_balance, get_p2pkh_address, send_from_p2pkh_address};

mod ethereum;
use ethereum::{get_address, transfer};

thread_local! {
    static API_KEY : RefCell<String> = const {RefCell::new(String::new())} ;
}

#[init]
fn init(api_key: String) {
    API_KEY.with_borrow_mut(|key| *key = api_key)
}

struct AlloyWallet;

#[derive(JsonSchema, Deserialize)]
enum Chain {
    Bitcoin,
    Ethereum,
    Solana,
}

#[derive(JsonSchema, Deserialize)]
struct GetAddressRequest {
    chain: Chain,
}

#[derive(JsonSchema, Deserialize)]
struct GetBalanceRequest {
    chain: Chain,
    address: String,
}

#[derive(JsonSchema, Deserialize)]
struct TransferRequest {
    chain: Chain,
    destination_address: String,
    amount: u128,
}

impl Handler for AlloyWallet {
    async fn call_tool(&self, req: CallToolRequestParam) -> Result<CallToolResult, Error> {
        match req.name.as_ref() {
            "get_address" => {
                match from_value::<GetAddressRequest>(Value::Object(req.arguments.ok_or(
                    Error::invalid_params("invalid arguments to tool get_address", None),
                )?))
                .map_err(|_| Error::invalid_params("invalid arguments to tool get_address", None))?
                .chain
                {
                    Chain::Bitcoin => Ok(CallToolResult::success(
                        Content::text(format!(
                            "The p2pkh legacy Bitcoin address of the server is: {}",
                            get_p2pkh_address()
                                .await
                                .map_err(|err| Error::internal_error(format!("{:?}", err), None))?
                        ))
                        .into_contents(),
                    )),
                    Chain::Ethereum => Ok(CallToolResult::success(
                        Content::text(format!(
                            "The EIP-55 checksum format Ethereum address of the server is: {}",
                            get_address()
                                .await
                                .map_err(|err| Error::internal_error(format!("{:?}", err), None))?
                        ))
                        .into_contents(),
                    )),
                    Chain::Solana => Ok(CallToolResult::error(
                        Content::text("Unimplemented.").into_contents(),
                    )),
                }
            }
            "get_balance" => {
                let args = from_value::<GetBalanceRequest>(Value::Object(req.arguments.ok_or(
                    Error::invalid_params("invalid arguments to tool get_balance", None),
                )?))
                .map_err(|_| {
                    Error::invalid_params("invalid arguments to tool get_balance", None)
                })?;
                match args.chain {
                    Chain::Bitcoin => Ok(CallToolResult::success(
                        Content::text(format!(
                            "The Bitcoin balance is: {} satoshi.",
                            get_balance(args.address)
                                .await
                                .map_err(|err| Error::internal_error(format!("{:?}", err), None))?
                        ))
                        .into_contents(),
                    )),
                    Chain::Ethereum => Ok(CallToolResult::success(
                        Content::text(format!(
                            "The Ethereum balance is: {} wei.",
                            ethereum::get_balance(args.address)
                                .await
                                .map_err(|err| Error::internal_error(format!("{:?}", err), None))?
                        ))
                        .into_contents(),
                    )),
                    Chain::Solana => Ok(CallToolResult::error(
                        Content::text("Unimplemented.").into_contents(),
                    )),
                }
            }
            "transfer" => {
                let args = from_value::<TransferRequest>(Value::Object(req.arguments.ok_or(
                    Error::invalid_params("invalid arguments to tool transfer", None),
                )?))
                .map_err(|_| Error::invalid_params("invalid arguments to tool transfer", None))?;

                match args.chain {
                    Chain::Bitcoin => Ok(CallToolResult::error(
                        Content::text(format!(
                            "Success! The transaction Id is {}",
                            send_from_p2pkh_address(args.destination_address, args.amount as u64)
                                .await
                                .map_err(|err| Error::internal_error(format!("{:?}", err), None))?
                        ))
                        .into_contents(),
                    )),
                    Chain::Ethereum => Ok(CallToolResult::error(
                        Content::text(format!(
                            "Success! The raw transaction hash is {}",
                            transfer(args.destination_address, args.amount)
                                .await
                                .map_err(|err| Error::internal_error(format!("{:?}", err), None))?
                        ))
                        .into_contents(),
                    )),
                    Chain::Solana => Ok(CallToolResult::error(
                        Content::text("Unimplemented.").into_contents(),
                    )),
                }
            }
            _ => Err(Error::invalid_params("not found tool", None)),
        }
    }
    async fn list_tools(&self, _: Option<PaginatedRequestParam>) -> Result<ListToolsResult, Error> {
        Ok(ListToolsResult {
            next_cursor: None,
            tools: vec![
                Tool::new(
                    "get_address",
                    "Get wallet (server) address for a given chain. Use this for topping up assets into wallet balance.",
                    schema_for_type::<GetAddressRequest>(),
                ),
                Tool::new(
                    "get_balance",
                    "Get a wallet's balance for a given chain.",
                    schema_for_type::<GetBalanceRequest>(),
                ),
                Tool::new(
                    "transfer",
                    "Transfer assets (coin/token) from the server address to a given address on a specified chain. For Bitcoin, the amount is in the unit of Satoshi. For Ethereum, it's Wei.",
                    schema_for_type::<TransferRequest>(),
                )
            ],
        })
    }
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "Alloy wallet server".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some(
                r"
            This server acts as a wallet supporting wallet management (get balance, transfer) for various chains (
            Bitcoin, Ethereum, and Solana).
            "
                .to_string(),
            ),
            ..Default::default()
        }
    }
}

#[query]
fn http_request(_: HttpRequest) -> HttpResponse {
    HttpResponse::builder()
        .with_status_code(StatusCode::OK)
        .with_upgrade(true)
        .build()
}

#[update]
async fn http_request_update(req: HttpRequest<'_>) -> HttpResponse {
    AlloyWallet {}
        .handle_with_auth(&req, |headers| {
            headers
                .iter()
                .any(|(k, v)| k == "x-api-key" && *v == API_KEY.with_borrow(|k| k.clone()))
        })
        .await
}

ic_cdk::export_candid!();

getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}
