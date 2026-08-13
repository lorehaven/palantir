pub mod node_conditions;
pub mod node_info;
pub mod node_pods;

use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::stats::{nodes as nodes_stat, pods as pods_stat};

pub async fn render(cache: &CacheStore, current_path: &str, node_name: &str) -> String {
    crate::shell::page(
        &["Cluster", "Nodes", node_name],
        current_path,
        div()
            .class("cluster-node main-page")
            .child(nodes_stat::render(cache, Some(node_name)).await)
            .child(pods_stat::render(cache, None, Some(node_name)).await)
            .child(node_info::fragment(cache, node_name).await)
            .child(node_conditions::fragment(cache, node_name).await)
            .child(node_pods::fragment(cache, node_name).await),
    )
}
