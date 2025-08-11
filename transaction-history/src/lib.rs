use candid::{Nat, Principal};
use ic_cdk::{init, query, update};
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use ic_rmcp::{model::*, schema_for_type, Context, Error, Handler, Server};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{from_value, Value};
use std::cell::RefCell;

mod icp_index;
use icp_index::{Account, GetAccountTransactionsArgs, Service};

thread_local! {
    static API_KEY : RefCell<String> = const {RefCell::new(String::new())} ;
}

#[init]
fn init(api_key: String) {
    API_KEY.with_borrow_mut(|key| *key = api_key)
}

#[derive(JsonSchema, Deserialize)]
struct GetTransactionHistoryRequest {
    principal: String,
    max_results: Option<u8>,
}

struct TransactionHistory;

impl Handler for TransactionHistory {
    fn get_info(&self, _: Context) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "Transaction history server".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some("This server provides tools to get ICP transaction history for default acount given a principal.".to_string()),
            ..Default::default()
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
                    "get_transaction_history",
                    "Return ICP transaction history (latest transactions) of the default account for principal. If max_results is empty, default to 5.",
                    schema_for_type::<GetTransactionHistoryRequest>(),
                ),
            ],
        })
    }

    async fn call_tool(
        &self,
        _: Context,
        request: CallToolRequestParam,
    ) -> Result<CallToolResult, Error> {
        match request.name.as_ref() {
            "get_transaction_history" => {
                let request = from_value::<GetTransactionHistoryRequest>(Value::Object(
                    request.arguments.ok_or(Error::invalid_params(
                        "invalid arguments to tool get_transaction_history",
                        None,
                    ))?,
                ))
                .map_err(|_| {
                    Error::invalid_params("invalid arguments to tool get_transaction_history", None)
                })?;

                let principal = Principal::from_text(request.principal).map_err(|_| {
                    Error::invalid_params("invalid arguments to tool get_transaction_history", None)
                })?;

                let index = Service(Principal::from_text("qhbym-qaaaa-aaaaa-aaafq-cai").unwrap());

                let response = index
                    .get_account_transactions(&GetAccountTransactionsArgs {
                        max_results: Nat::from(request.max_results.unwrap_or(5)),
                        start: None,
                        account: Account {
                            owner: principal,
                            subaccount: None,
                        },
                    })
                    .await
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?
                    .0
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

                let content = Content::json(response)
                    .map_err(|err| Error::internal_error(format!("{err:?}"), None))?;

                Ok(CallToolResult::success(content.into_contents()))
            }
            _ => Err(Error::invalid_params("not found tool", None)),
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
    TransactionHistory {}
        .handle(&req, |headers| {
            headers
                .iter()
                .any(|(k, v)| k == "x-api-key" && *v == API_KEY.with_borrow(|k| k.clone()))
        })
        .await
}

ic_cdk::export_candid!();
