use domain::cluster::node::Node;
#[allow(unused_imports)]
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::utils::{kube_api_request, ApiType};

pub async fn get_nodes_filtered(node_name: Option<String>) -> Vec<Node> {
    if let Some(name) = node_name {
        vec![get_node_by_name(name.clone()).await.unwrap_or_default()]
    } else {
        get_nodes().await.unwrap_or_default()
    }
}

#[server(GetNodesResponse, "/api/nodes/response")]
pub async fn get_nodes_response() -> Result<Response<Node>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "nodes".to_string()).await?;
    Ok(serde_json::from_str::<Response<Node>>(&response)?)
}

#[server(GetNodes, "/api/nodes")]
pub async fn get_nodes() -> Result<Vec<Node>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "nodes".to_string()).await?;
    Ok(serde_json::from_str::<Response<Node>>(&response)?.items)
}

#[server(GetNodeByName, "/api/node/:name")]
pub async fn get_node_by_name(name: String) -> Result<Node, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "nodes".to_string()).await?;
    let node = serde_json::from_str::<Response<Node>>(&response)?
        .items
        .iter()
        .find(|n| n.metadata.name == name)
        .cloned()
        .unwrap_or_default();
    Ok(node)
}
