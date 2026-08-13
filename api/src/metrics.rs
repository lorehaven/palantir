use domain::metrics::{NodeMetrics, PodMetrics};
use quench_cache::CacheStore;
use serde_json::Value;

pub async fn get_nodes(cache: &CacheStore) -> anyhow::Result<Vec<NodeMetrics>> {
    let metrics = get_metrics(cache, "nodes").await?;
    let items = metrics
        .get("items")
        .ok_or_else(|| anyhow::anyhow!("metrics response missing 'items'"))?;
    Ok(serde_json::from_value::<Vec<NodeMetrics>>(items.clone())?)
}

pub async fn get_pods(cache: &CacheStore) -> anyhow::Result<Vec<PodMetrics>> {
    let metrics = get_metrics(cache, "pods").await?;
    let items = metrics
        .get("items")
        .ok_or_else(|| anyhow::anyhow!("metrics response missing 'items'"))?;
    Ok(serde_json::from_value::<Vec<PodMetrics>>(items.clone())?)
}

async fn get_metrics(cache: &CacheStore, path: &str) -> anyhow::Result<Value> {
    let server_host = crate::config::server_host();
    let server_port = crate::config::server_port();

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!(
            "https://{server_host}:{server_port}/apis/metrics.k8s.io/v1beta1/{path}"
        ))
        .bearer_auth(crate::utils::get_api_token(cache).await)
        .send()
        .await?;

    response.error_for_status_ref()?;
    Ok(response.json().await?)
}
