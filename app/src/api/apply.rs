use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::{get_api_token, get_url};

#[server(Apply, "/api/apply")]
pub async fn apply(payload: String) -> Result<String, ServerFnError> {
    let url = get_url(payload.clone()).await?;
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .post(format!("https://{server_host}:6443/{url}"))
        .header("Content-Type", "application/json")
        .body(payload)
        .bearer_auth(get_api_token())
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(ServerFnError::ServerError(response.status().to_string()))
    }
}
