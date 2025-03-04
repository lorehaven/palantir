use leptos::prelude::ServerFnError;
use leptos::server;

use crate::domain::node::{Node, NodesResponse};

#[server(GetNodes, "/api/nodes")]
pub async fn get_nodes(node_name: Option<String>) -> Result<Vec<Node>, ServerFnError> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let token = crate::api::utils::get_api_token();

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!("https://{server_host}:6443/api/v1/nodes"))
        .bearer_auth(token)
        .send()
        .await?;

    response.error_for_status_ref()?;
    Ok(parse_response(&response.text().await?, node_name).unwrap_or_default())
}

#[allow(dead_code)]
fn parse_response(response: &str, node_name: Option<String>) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
    let mut nodes = serde_json::from_str::<NodesResponse>(response)?.items;
    if let Some(node_name) = node_name {
        nodes.retain(|n| n.metadata.name == node_name)
    }
    Ok(nodes)
}
