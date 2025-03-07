use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::get_api_token;
use crate::domain::node::{Node, NodesResponse};

#[server]
async fn get_nodes_internal() -> Result<String, ServerFnError> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!("https://{server_host}:6443/api/v1/nodes"))
        .bearer_auth(get_api_token())
        .send()
        .await?;

    response.error_for_status_ref()?;
    Ok(response.text().await?)
}

#[server(GetNodesResponse, "/api/nodes/response")]
pub async fn get_nodes_response() -> Result<NodesResponse, ServerFnError> {
    let response = get_nodes_internal().await?;
    Ok(serde_json::from_str::<NodesResponse>(&response)?)
}

#[server(GetNodes, "/api/nodes")]
pub async fn get_nodes() -> Result<Vec<Node>, ServerFnError> {
    let response = get_nodes_internal().await?;
    Ok(serde_json::from_str::<NodesResponse>(&response)?.items)
}

#[server(GetNodeByName, "/api/node/:name")]
pub async fn get_node_by_name(name: String) -> Result<Node, ServerFnError> {
    let response = get_nodes_internal().await?;
    let node = serde_json::from_str::<NodesResponse>(&response)?.items
        .iter()
        .find(|n| n.metadata.name == name)
        .cloned()
        .unwrap_or_default();
    Ok(node.clone())
}
