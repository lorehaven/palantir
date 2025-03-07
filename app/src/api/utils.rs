use leptos::prelude::ServerFnError;
use leptos::server;

const DEFAULT_TOKEN_PATH: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";

#[allow(dead_code)]
pub fn get_api_token() -> String {
    let token_path = std::env::var("KUBERNETES_TOKEN_PATH").unwrap_or_else(|_| DEFAULT_TOKEN_PATH.to_string());
    std::fs::read_to_string(token_path)
        .expect("token file is missing.")
        .trim()
        .to_string()
}

#[server]
pub async fn kube_api_request(endpoint: String) -> Result<String, ServerFnError> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!("https://{server_host}:6443/api/v1/{endpoint}"))
        .bearer_auth(get_api_token())
        .send()
        .await?;

    response.error_for_status_ref()?;
    Ok(response.text().await?)
}
