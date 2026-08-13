use domain::shared::scale::Scale;
use quench_auth::prelude::{Claims, JwtConfig};
use quench_cache::CacheStore;

/// Short enough that a click-driven action (delete, scale, apply) is never
/// hidden behind a stale read for long, but still enough to dedupe the
/// bursts of near-simultaneous `get` calls one page render produces (list
/// pages resolve several resources' details in parallel) and to take the
/// edge off the periodic polling every page does.
const GET_CACHE_TTL_SECS: u64 = 5;

pub async fn get(
    cache: &CacheStore,
    resource_type: &str,
    namespace: Option<String>,
    resource: Option<String>,
) -> anyhow::Result<String> {
    let url = crate::utils::get_url(resource_type, namespace, resource)?;

    if let Ok(Some(cached)) = cache.get(&url).await {
        if let Some(body) = cached.as_str() {
            return Ok(body.to_string());
        }
    }

    let body = fetch(cache, &url).await?;
    let _ = cache
        .set(
            &url,
            serde_json::Value::String(body.clone()),
            Some(GET_CACHE_TTL_SECS),
        )
        .await;
    Ok(body)
}

async fn fetch(cache: &CacheStore, url: &str) -> anyhow::Result<String> {
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;
    let server_host = crate::config::server_host();
    let server_port = crate::config::server_port();

    let response = client
        .get(format!("https://{server_host}:{server_port}/{url}"))
        .bearer_auth(crate::utils::get_api_token(cache).await)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(anyhow::anyhow!(response.status().to_string()))
    }
}

pub async fn delete(
    cache: &CacheStore,
    config: &JwtConfig,
    claims: Option<&Claims>,
    resource_type: &str,
    namespace: Option<String>,
    resource: Option<String>,
) -> anyhow::Result<String> {
    crate::auth::require_write(config, claims)?;

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = crate::utils::get_url(resource_type, namespace, resource)?;
    let server_host = crate::config::server_host();
    let server_port = crate::config::server_port();

    let response = client
        .delete(format!("https://{server_host}:{server_port}/{url}"))
        .bearer_auth(crate::utils::get_api_token(cache).await)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(anyhow::anyhow!(response.status().to_string()))
    }
}

pub async fn logs(
    cache: &CacheStore,
    resource_type: &str,
    namespace: String,
    resource: String,
    container: String,
    previous: bool,
    tail_lines: i64,
) -> anyhow::Result<Vec<String>> {
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = crate::utils::get_url(resource_type, Some(namespace), Some(resource))?;
    let server_host = crate::config::server_host();
    let server_port = crate::config::server_port();

    let tail_lines = if tail_lines > 0 {
        format!("&tailLines={tail_lines}")
    } else {
        String::new()
    };
    let response = client
        .get(format!("https://{server_host}:{server_port}/{url}/log?container={container}&follow=false&previous={previous}{tail_lines}"))
        .bearer_auth(crate::utils::get_api_token(cache).await)
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
        Err(anyhow::anyhow!(response.status().to_string()))
    }
}

pub async fn scale(
    cache: &CacheStore,
    config: &JwtConfig,
    claims: Option<&Claims>,
    resource_type: &str,
    namespace: Option<String>,
    resource: Option<String>,
    replicas: i64,
) -> anyhow::Result<String> {
    crate::auth::require_write(config, claims)?;

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = crate::utils::get_url(resource_type, namespace.clone(), resource.clone())?;
    let server_host = crate::config::server_host();
    let server_port = crate::config::server_port();

    let response = client
        .put(format!("https://{server_host}:{server_port}/{url}/scale"))
        .bearer_auth(crate::utils::get_api_token(cache).await)
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
        Err(anyhow::anyhow!(response.status().to_string()))
    }
}
