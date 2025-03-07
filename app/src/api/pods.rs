use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::get_api_token;
use crate::domain::pod::*;

#[server(GetPods, "/api/pods")]
pub async fn get_pods() -> Result<Vec<Pod>, ServerFnError> {
    let response = get_pods_internal().await?;
    Ok(serde_json::from_str::<PodsResponse>(&response)?.items)
}

#[server(GetPodsByNodeName, "/api/pods/:node_name")]
pub async fn get_pods_by_node_name(node_name: String) -> Result<Vec<Pod>, ServerFnError> {
    let response = get_pods_internal().await?;
    let pods = serde_json::from_str::<PodsResponse>(&response)?.items
        .into_iter()
        .filter(|p| p.spec.node_name == node_name)
        .collect();
    Ok(pods)
}

#[server]
async fn get_pods_internal() -> Result<String, ServerFnError> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!("https://{server_host}:6443/api/v1/pods"))
        .bearer_auth(get_api_token())
        .send()
        .await?;

    response.error_for_status_ref()?;
    Ok(response.text().await?)
}
