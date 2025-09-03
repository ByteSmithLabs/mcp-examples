use candid::CandidType;
use ic_cdk::{init, query, update};
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use ic_rmcp::{
    model::*, schema_for_type, Context, Error, Handler, IssuerConfig, OAuthConfig, Server,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::cell::RefCell;

// TODO: data retention!
thread_local! {
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
    ARGS.with_borrow_mut(|args| *args = config);
}

#[derive(JsonSchema, Deserialize)]
struct StartRequest {
    // TODO: amount
}

#[derive(JsonSchema, Deserialize)]
struct PlayRequest {
    // TODO: game_hash.
}

struct OddEven;

impl Handler for OddEven {
    async fn call_tool(
        &self,
        _: Context,
        req: CallToolRequestParam,
    ) -> Result<CallToolResult, Error> {
        match req.name.as_ref() {
            "start" => Ok(CallToolResult::success(
                // TODO: implement this
                Content::text("Implement this").into_contents(),
            )),
            "play" => Ok(CallToolResult::success(
                // TODO: implement this
                Content::text("Implement this").into_contents(),
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
                    "start",
                    // TODO: Clarify more about input and output.
                    // Accept amount to deduct money.
                    // Save game info.
                    // Return game hash. Hashed from result (odd or even) + random number + created timestamp
                    "Start a odd-even game.",
                    schema_for_type::<StartRequest>(),
                ),
                Tool::new(
                    "play",
                    // TODO: Clarify more about input and output.
                    // Accept game hash, retrieve game info. Check if exist.
                    // Check timestamp.
                    // Compare.
                    // Disburse if needed.
                    "Play a odd-even game.",
                    schema_for_type::<PlayRequest>(),
                ),
            ],
        })
    }
    fn get_info(&self, _: Context) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "Odd-even".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some(
                r"
            This server provides tools for users to play odd-even game. 
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
    OddEven {}
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
