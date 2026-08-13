use domain::cluster::node::Node;
use domain::shared::response::Response;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_nodes_filtered(cache: &CacheStore, node_name: Option<String>) -> Vec<Node> {
    if let Some(name) = node_name {
        vec![get_node_by_name(cache, name.clone())
            .await
            .unwrap_or_default()]
    } else {
        get_nodes(cache).await.unwrap_or_default()
    }
}

pub async fn get_nodes_response(cache: &CacheStore) -> anyhow::Result<Response<Node>> {
    let response = resource_api::get(cache, "Node", None, None).await?;
    Ok(serde_json::from_str::<Response<Node>>(&response)?)
}

pub async fn get_nodes(cache: &CacheStore) -> anyhow::Result<Vec<Node>> {
    let response = resource_api::get(cache, "Node", None, None).await?;
    Ok(serde_json::from_str::<Response<Node>>(&response)?.items)
}

pub async fn get_node_by_name(cache: &CacheStore, name: String) -> anyhow::Result<Node> {
    let response = resource_api::get(cache, "Node", None, Some(name)).await?;
    Ok(serde_json::from_str::<Node>(&response).unwrap_or_default())
}
