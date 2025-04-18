use domain::cluster::node::Node;
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

pub async fn get_nodes_filtered(node_name: Option<String>) -> Vec<Node> {
    if let Some(name) = node_name {
        vec![get_node_by_name(name.clone()).await.unwrap_or_default()]
    } else {
        get_nodes().await.unwrap_or_default()
    }
}

#[server(GetNodesResponse, "/api/nodes/response")]
pub async fn get_nodes_response() -> Result<Response<Node>, ServerFnError> {
    let response = resource_api::get("Node".to_string(), None, None).await?;
    Ok(serde_json::from_str::<Response<Node>>(&response)?)
}

#[server(GetNodes, "/api/nodes")]
pub async fn get_nodes() -> Result<Vec<Node>, ServerFnError> {
    let response = resource_api::get("Node".to_string(), None, None).await?;
    Ok(serde_json::from_str::<Response<Node>>(&response)?.items)
}

#[server(GetNodeByName, "/api/node/:name")]
pub async fn get_node_by_name(name: String) -> Result<Node, ServerFnError> {
    let response = resource_api::get("Node".to_string(), None, Some(name)).await?;
    Ok(serde_json::from_str::<Node>(&response).unwrap_or_default())
}
