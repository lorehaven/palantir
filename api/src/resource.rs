use leptos::prelude::ServerFnError;
use leptos::server;
use serde_json::Value;
use domain::shared::response::Response;
use domain::shared::scale::Scale;
use crate::utils::{get_api_token, get_url, ApiMode};

#[server(GetResource, "/api/resources/get")]
pub async fn get(
    resource_type: String,
    namespace: Option<String>,
    resource: Option<String>,
) -> Result<String, ServerFnError> {
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = get_url(resource_type, namespace, resource).await?;
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let response = client
        .get(format!("https://{server_host}:6443/{url}"))
        .bearer_auth(get_api_token())
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(ServerFnError::ServerError(response.status().to_string()))
    }
}

#[server(DeleteResource, "/api/resources/delete")]
pub async fn delete(
    resource_type: String,
    namespace: Option<String>,
    resource: Option<String>,
) -> Result<String, ServerFnError> {
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = get_url(resource_type, namespace, resource).await?;
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let response = client
        .delete(format!("https://{server_host}:6443/{url}"))
        .bearer_auth(get_api_token())
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(ServerFnError::ServerError(response.status().to_string()))
    }
}

#[server(ResourceLogs, "/api/resources/logs")]
pub async fn logs(
    resource_type: String,
    namespace: String,
    resource: String,
    container: String,
    previous: bool,
    tail_lines: i64,
) -> Result<Vec<String>, ServerFnError> {
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = get_url(resource_type, Some(namespace), Some(resource)).await?;
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let tail_lines = if tail_lines > 0 { format!("&tailLines={tail_lines}") } else { String::new() };
    let response = client
        .get(format!("https://{server_host}:6443/{url}/log?container={container}&follow=false&previous={previous}{tail_lines}"))
        .bearer_auth(get_api_token())
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.text().await?
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(ToOwned::to_owned)
            .collect())
    } else {
        Err(ServerFnError::ServerError(response.status().to_string()))
    }
}

#[server(ScaleResource, "/api/resources/scale")]
pub async fn scale(
    resource_type: String,
    namespace: Option<String>,
    resource: Option<String>,
    replicas: i64,
) -> Result<String, ServerFnError> {
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = get_url(resource_type, namespace.clone(), resource.clone()).await?;
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let response = client
        .put(format!("https://{server_host}:6443/{url}/scale"))
        .bearer_auth(get_api_token())
        .body(serde_json::to_string(&Scale::new(
            &namespace.unwrap_or_default(),
            &resource.unwrap_or_default(),
            replicas,
        )).unwrap_or_default())
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(ServerFnError::ServerError(response.status().to_string()))
    }
}
