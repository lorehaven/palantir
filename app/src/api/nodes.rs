use leptos::prelude::ServerFnError;
use leptos::server;

#[server(GetNodes, "/api/nodes")]
pub async fn get_nodes() -> Result<String, ServerFnError> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let token = crate::api::utils::get_api_token();

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!("https://{server_host}:6443/api/v1/nodes"))
        .bearer_auth(token)
        .send()
        .await?;

    response.error_for_status_ref()?;
    Ok(response.text().await?)
}
