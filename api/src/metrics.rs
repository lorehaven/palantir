use domain::metrics::{NodeMetrics, PodMetrics};
use leptos::prelude::ServerFnError;
use leptos::server;
use serde_json::Value;

use crate::utils::get_api_token;

#[server(GetNodesMetrics, "/api/metrics/nodes")]
pub async fn get_nodes() -> Result<Vec<NodeMetrics>, ServerFnError> {
    let metrics = get_metrics("nodes".to_string()).await?;
    serde_json::from_value::<Vec<NodeMetrics>>(metrics.get("items").unwrap().clone())
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(GetPodsMetrics, "/api/metrics/pods")]
pub async fn get_pods() -> Result<Vec<PodMetrics>, ServerFnError> {
    let metrics = get_metrics("pods".to_string()).await?;
    serde_json::from_value::<Vec<PodMetrics>>(metrics.get("items").unwrap().clone())
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
async fn get_metrics(path: String) -> Result<Value, ServerFnError> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!(
            "https://{server_host}:6443/apis/metrics.k8s.io/v1beta1/{path}"
        ))
        .bearer_auth(get_api_token())
        .send()
        .await?;

    response.error_for_status_ref()?;
    Ok(response.json().await?)
}
