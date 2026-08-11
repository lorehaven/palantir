use domain::shared::scale::Scale;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::utils::{get_api_token, get_url};

/// Short enough that a click-driven action (delete, scale, apply) is never
/// hidden behind a stale read for long, but still enough to dedupe the
/// bursts of near-simultaneous `get` calls one page render produces (list
/// pages resolve several resources' details in parallel) and to take the
/// edge off the periodic polling every page does.
#[cfg(not(target_arch = "wasm32"))]
const GET_CACHE_TTL_SECS: u64 = 5;

#[server(GetResource, "/api/resources/get")]
pub async fn get(
    resource_type: String,
    namespace: Option<String>,
    resource: Option<String>,
) -> Result<String, ServerFnError> {
    let url = get_url(resource_type, namespace, resource).await?;

    #[cfg(not(target_arch = "wasm32"))]
    if let Ok(store) =
        leptos_actix::extract::<actix_web::web::Data<quench_cache::CacheStore>>().await
    {
        if let Ok(Some(cached)) = store.get(&url).await {
            if let Some(body) = cached.as_str() {
                return Ok(body.to_string());
            }
        }

        let body = fetch(&url).await?;
        let _ = store
            .set(
                &url,
                serde_json::Value::String(body.clone()),
                Some(GET_CACHE_TTL_SECS),
            )
            .await;
        return Ok(body);
    }

    fetch(&url).await
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch(url: &str) -> Result<String, ServerFnError> {
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;
    let server_host = crate::config::server_host();
    let server_port = crate::config::server_port();

    let response = client
        .get(format!("https://{server_host}:{server_port}/{url}"))
        .bearer_auth(get_api_token().await)
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
    crate::auth::require_write().await?;

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = get_url(resource_type, namespace, resource).await?;
    let server_host = crate::config::server_host();
    let server_port = crate::config::server_port();

    let response = client
        .delete(format!("https://{server_host}:{server_port}/{url}"))
        .bearer_auth(get_api_token().await)
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
    let server_host = crate::config::server_host();
    let server_port = crate::config::server_port();

    let tail_lines = if tail_lines > 0 {
        format!("&tailLines={tail_lines}")
    } else {
        String::new()
    };
    let response = client
        .get(format!("https://{server_host}:{server_port}/{url}/log?container={container}&follow=false&previous={previous}{tail_lines}"))
        .bearer_auth(get_api_token().await)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response
            .text()
            .await?
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
    crate::auth::require_write().await?;

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = get_url(resource_type, namespace.clone(), resource.clone()).await?;
    let server_host = crate::config::server_host();
    let server_port = crate::config::server_port();

    let response = client
        .put(format!("https://{server_host}:{server_port}/{url}/scale"))
        .bearer_auth(get_api_token().await)
        .body(
            serde_json::to_string(&Scale::new(
                &namespace.unwrap_or_default(),
                &resource.unwrap_or_default(),
                replicas,
            ))
            .unwrap_or_default(),
        )
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(ServerFnError::ServerError(response.status().to_string()))
    }
}
