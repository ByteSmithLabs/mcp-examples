use ic_cdk::api;
use ic_cdk::{init, query, update};
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use ic_rmcp::{model::*, schema_for_type, Context, Error, Handler, Server};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{from_value, Value};
use std::cell::RefCell;

use crate::ledger::{Account, Service, TransferArg};
use candid::{Nat, Principal};

mod ledger;

thread_local! {
    static TOKENS: RefCell<Vec<(String, String, u8)>> = const{RefCell::new(Vec::new())};
    static API_KEY : RefCell<String> = const {RefCell::new(String::new())} ;
}

#[init]
fn init(api_key: String) {
    TOKENS.with_borrow_mut(|tokens| {
        tokens.push((
            "ICP".to_string(),
            "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(),
            8,
        ));
        tokens.push((
            "ckBTC".to_string(),
            "mxzaz-hqaaa-aaaar-qaada-cai".to_string(),
            8,
        ));
        tokens.push((
            "ckETH".to_string(),
            "ss2fx-dyaaa-aaaar-qacoq-cai".to_string(),
            18,
        ));
        tokens.push((
            "ckUSDC".to_string(),
            "xevnm-gaaaa-aaaar-qafnq-cai".to_string(),
            6,
        ))
    });

    API_KEY.with_borrow_mut(|key| *key = api_key)
}

struct TokenTransferring;

#[derive(JsonSchema, Deserialize)]
struct TransferRequest {
    to: String,
    amount: u64,
    ledger_canister_id: String,
}

#[derive(JsonSchema, Deserialize)]
struct GetBalanceRequest {
    owner: Option<String>,
    ledger_canister_id: String,
}

#[derive(JsonSchema, Deserialize)]
struct AddTokenRequest {
    name: String,
    ledger_canister_id: String,
}

impl Handler for TokenTransferring {
    async fn call_tool(
        &self,
        _: Context,
        req: CallToolRequestParam,
    ) -> Result<CallToolResult, Error> {
        match req.name.as_ref() {
            "get_balance" => {
                let request = from_value::<GetBalanceRequest>(Value::Object(req.arguments.ok_or(
                    Error::invalid_params("invalid arguments to tool get_balance", None),
                )?))
                .map_err(|_| {
                    Error::invalid_params("invalid arguments to tool get_balance", None)
                })?;

                let ledger = Service(
                    Principal::from_text(request.ledger_canister_id)
                        .map_err(|_| Error::invalid_params("invalid ledger canister id", None))?,
                );

                let (symbol,) = ledger
                    .icrc_1_symbol()
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

                let (decimals,) = ledger
                    .icrc_1_decimals()
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

                let owner = if let Some(own_text) = request.owner {
                    Principal::from_text(own_text).map_err(|_| {
                        Error::invalid_params("invalid arguments to tool get_balance", None)
                    })?
                } else {
                    api::canister_self()
                };

                let balance = ledger
                    .icrc_1_balance_of(&Account {
                        owner,
                        subaccount: None,
                    })
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

                Ok(CallToolResult::success(
                    Content::text(format!(
                        "The balance is {} {}",
                        Decimal::from_i128_with_scale(
                            i128::try_from(balance.0 .0).unwrap(),
                            decimals as u32
                        ),
                        symbol
                    ))
                    .into_contents(),
                ))
            }
            "transfer" => {
                let req = from_value::<TransferRequest>(Value::Object(req.arguments.ok_or(
                    Error::invalid_params("invalid arguments to tool transfer", None),
                )?))
                .map_err(|_| Error::invalid_params("invalid arguments to tool transfer", None))?;

                let recipiant = Principal::from_text(req.to).map_err(|_| {
                    Error::invalid_params("invalid principal in tool transfer", None)
                })?;

                let ledger = Service(
                    Principal::from_text(req.ledger_canister_id)
                        .map_err(|_| Error::invalid_params("invalid ledger canister id", None))?,
                );

                let _ = ledger
                    .icrc_1_transfer(&TransferArg {
                        to: Account {
                            owner: recipiant,
                            subaccount: None,
                        },
                        fee: None,
                        memo: None,
                        from_subaccount: None,
                        created_at_time: None,
                        amount: Nat::from(req.amount),
                    })
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?
                    .0
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

                Ok(CallToolResult::success(
                    Content::text("Success").into_contents(),
                ))
            }
            "get_principal" => Ok(CallToolResult::success(
                Content::text(format!("Canister principal: {}", api::canister_self()))
                    .into_contents(),
            )),
            "add_token" => {
                let request = from_value::<AddTokenRequest>(Value::Object(req.arguments.ok_or(
                    Error::invalid_params("invalid arguments to tool add_token", None),
                )?))
                .map_err(|_| Error::invalid_params("invalid arguments to tool add_token", None))?;

                let ledger = Service(
                    Principal::from_text(&request.ledger_canister_id)
                        .map_err(|_| Error::invalid_params("invalid ledger canister id", None))?,
                );

                let decimals = ledger
                    .icrc_1_decimals()
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?
                    .0;

                TOKENS.with_borrow_mut(|tokens| {
                    tokens.push((request.name, request.ledger_canister_id, decimals));
                });
                Ok(CallToolResult::success(
                    Content::text("Success").into_contents(),
                ))
            }
            "get_supported_tokens" => Ok(CallToolResult::success(
                Content::text(format!("{:?}", TOKENS.with_borrow(|tokens| tokens.clone())))
                    .into_contents(),
            )),
            _ => Err(Error::invalid_params("not found tool", None)),
        }
    }
    async fn list_tools(
        &self,
        _: Context,
        _: Option<PaginatedRequestParam>,
    ) -> Result<ListToolsResult, Error> {
        Ok(ListToolsResult {
            next_cursor: None,
            tools: vec![
                Tool::new(
                    "get_balance",
                    "Get ICRC-1 token balance for the given account. It accepts two parameters: textual principal and ledger canister ID. If principal is empty, default to canister (server) account.",
                    schema_for_type::<GetBalanceRequest>(),
                ),
                Tool::new("transfer",
                 "Transfer ICRC-1 token from the canister (server) account to the given destination principal. The unit is in decimal format. For example, to transfer 1.5 ICP, you should pass 150000000 as amount. To get decimals value, use get_supported_tokens tool.",
                schema_for_type::<TransferRequest>()),
                Tool::new("get_principal", "Get the canister (server) default account in textual principal. Use this tool when you need to top up ICRC-1 token to your canister (server).", schema_for_type::<EmptyObject>()),
                Tool::new("add_token", "Add new token to token list.", schema_for_type::<AddTokenRequest>()),
                Tool::new("get_supported_tokens", "Return a list of supported tokens. Use this when retrieving token's ledger canister ID and its decimals.", schema_for_type::<EmptyObject>())
            ],
        })
    }
    fn get_info(&self, _: Context) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "Token transfer server".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some(
                r"
            This server provides tools to transfer ICRC-1 tokens from the canister account (default 
            account; represented by the server) to destination account.
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
    TokenTransferring {}
        .handle(&req, |headers| {
            headers
                .iter()
                .any(|(k, v)| k == "x-api-key" && *v == API_KEY.with_borrow(|k| k.clone()))
        })
        .await
}

ic_cdk::export_candid!();
