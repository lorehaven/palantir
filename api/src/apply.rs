use leptos::prelude::ServerFnError;
use leptos::server;
use crate::utils::{get_api_token, get_url, ApiMode};

#[server(Apply, "/api/apply")]
pub async fn apply(payload: String, mode: ApiMode) -> Result<String, ServerFnError> {
    let payload_json = serde_json::from_str::<serde_json::Value>(&payload)
        .map_err(|e| ServerFnError::new(format!("Failed to parse JSON: {e}")))?;
    let resource_type = payload_json.get("kind")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| ServerFnError::new("Missing or invalid 'kind' field"))?
        .to_string();
    let metadata = payload_json.get("metadata")
        .ok_or_else(|| ServerFnError::new("Missing 'metadata' field"))?;
    let namespace = metadata.get("namespace")
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string);
    let resource = (mode == ApiMode::Put)
        .then(|| {
            metadata.get("name")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| ServerFnError::new("Missing or invalid 'name' field"))
                .map(ToString::to_string)
        })
        .transpose()?;

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = get_url(resource_type, namespace, resource).await?;
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let response = match mode {
        ApiMode::Post => client
            .post(format!("https://{server_host}:6443/{url}")),
        ApiMode::Put => client
            .put(format!("https://{server_host}:6443/{url}")),
        _ => unimplemented!(),
    }
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
