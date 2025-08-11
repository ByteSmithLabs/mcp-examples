use candid::CandidType;
use candid::{Nat, Principal};
use ic_cdk::api;
use ic_cdk::{init, query, update};
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use ic_rmcp::{
    model::*, schema_for_type, Context, Error, Handler, IssuerConfig, OAuthConfig, Server,
};
use icrc_ledger_client::ICRC1Client;
use icrc_ledger_types::icrc1::{
    account::{principal_to_subaccount, Account},
    transfer::TransferArg,
};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{from_value, Value};
use std::cell::RefCell;

mod runtime;

use runtime::CdkRuntime;

thread_local! {
    static TOKENS: RefCell<Vec<(String, String, u8)>> = const{RefCell::new(Vec::new())};
    static ARGS : RefCell<InitArgs> =  RefCell::default();
}

#[derive(Deserialize, CandidType, Default)]
struct InitArgs {
    metadata_url: String,
    resource: String,
    issuer: String,
    jwks_url: String,
    authorization_server: Vec<String>,
    audience: String,
}

#[init]
fn init(config: InitArgs) {
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

    ARGS.with_borrow_mut(|args| *args = config);
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
        context: Context,
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

                let client = ICRC1Client {
                    runtime: CdkRuntime,
                    ledger_canister_id: Principal::from_text(request.ledger_canister_id)
                        .map_err(|_| Error::invalid_params("invalid ledger canister id", None))?,
                };

                let symbol = client
                    .symbol()
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

                let decimals = client
                    .decimals()
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

                let subject = context
                    .subject
                    .ok_or(Error::internal_error("no subject".to_string(), None))?;

                let subaccount = principal_to_subaccount(
                    Principal::from_text(subject)
                        .map_err(|err| Error::internal_error(format!("{err:?}"), None))?,
                );

                let balance = client
                    .balance_of(Account {
                        owner: api::canister_self(),
                        subaccount: Some(subaccount),
                    })
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

                Ok(CallToolResult::success(
                    Content::text(format!(
                        "The balance is {} {}",
                        Decimal::from_i128_with_scale(
                            i128::try_from(balance.0).unwrap(),
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

                let client = ICRC1Client {
                    runtime: CdkRuntime,
                    ledger_canister_id: Principal::from_text(req.ledger_canister_id)
                        .map_err(|_| Error::invalid_params("invalid ledger canister id", None))?,
                };

                let subject = context
                    .subject
                    .ok_or(Error::internal_error("no subject".to_string(), None))?;

                let subaccount = principal_to_subaccount(
                    Principal::from_text(subject)
                        .map_err(|err| Error::internal_error(format!("{err:?}"), None))?,
                );

                let _ = client
                    .transfer(TransferArg {
                        to: Account {
                            owner: recipiant,
                            subaccount: None,
                        },
                        fee: None,
                        memo: None,
                        from_subaccount: Some(subaccount),
                        created_at_time: None,
                        amount: Nat::from(req.amount),
                    })
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

                Ok(CallToolResult::success(
                    Content::text("Success").into_contents(),
                ))
            }
            "get_account_address" => {
                let subject = context
                    .subject
                    .ok_or(Error::internal_error("no subject".to_string(), None))?;

                let subaccount = principal_to_subaccount(
                    Principal::from_text(subject)
                        .map_err(|err| Error::internal_error(format!("{err:?}"), None))?,
                );

                Ok(CallToolResult::success(
                    Content::text(format!(
                        "Address: {}",
                        Account {
                            owner: api::canister_self(),
                            subaccount: Some(subaccount),
                        }
                    ))
                    .into_contents(),
                ))
            }
            "add_token" => {
                let request = from_value::<AddTokenRequest>(Value::Object(req.arguments.ok_or(
                    Error::invalid_params("invalid arguments to tool add_token", None),
                )?))
                .map_err(|_| Error::invalid_params("invalid arguments to tool add_token", None))?;

                let client = ICRC1Client {
                    runtime: CdkRuntime,
                    ledger_canister_id: Principal::from_text(request.ledger_canister_id.clone())
                        .map_err(|_| Error::invalid_params("invalid ledger canister id", None))?,
                };

                let decimals = client
                    .decimals()
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

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
                    "Get ICRC-1 token balance for your account. The subaccount will be derived from user's authenticated identity.",
                    schema_for_type::<GetBalanceRequest>(),
                ),
                Tool::new("transfer",
                 "Transfer ICRC-1 token from your account to the given destination principal. The unit is in decimal format. For example, to transfer 1.5 ICP, you should pass 150000000 as amount. To get decimals value, use get_supported_tokens tool.",
                schema_for_type::<TransferRequest>()),
                Tool::new("get_account_address", "Get your address. Basically, it consists of a subaccount mapped from authenticated identity, under the server principal. Use this tool when you need to top up ICRC-1 token to your account.", schema_for_type::<EmptyObject>()),
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
            This server provides tools to transfer ICRC-1 tokens user's account to destination account.
            Each user will control a subaccount under canister (server) principal. 
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
async fn http_request_update(req: HttpRequest<'_>) -> HttpResponse<'_> {
    TokenTransferring {}
        .handle_with_oauth(
            &req,
            ARGS.with_borrow(|args| OAuthConfig {
                metadata_url: args.metadata_url.clone(),
                resource: args.resource.clone(),
                issuer_configs: IssuerConfig {
                    issuer: args.issuer.clone(),
                    jwks_url: args.jwks_url.clone(),
                    authorization_server: args.authorization_server.clone(),
                    audience: args.audience.clone(),
                },
                scopes_supported: vec![],
            }),
        )
        .await
}

ic_cdk::export_candid!();
