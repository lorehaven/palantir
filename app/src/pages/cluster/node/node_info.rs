use api::cluster::nodes as nodes_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn fragment(cache: &CacheStore, node_name: &str) -> Element {
    let response = nodes_api::get_nodes_response(cache)
        .await
        .unwrap_or_default();
    let kind = if response.kind == "NodesList" {
        "Node".to_string()
    } else {
        response.kind
    };
    let node = response
        .items
        .into_iter()
        .find(|n| n.metadata.name == node_name)
        .unwrap_or_default();

    let data = vec![
        ("Name".to_string(), node.metadata.name.clone()),
        ("Kind".to_string(), kind),
        (
            "Created".to_string(),
            format_timestamp(
                node.metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        ("Labels".to_string(), display::hashmap(node.metadata.labels)),
        (
            "Annotations".to_string(),
            display::hashmap(node.metadata.annotations),
        ),
        ("Version".to_string(), node.metadata.resource_version),
        (
            "Kernel Version".to_string(),
            node.status.node_info.kernel_version,
        ),
        ("OS".to_string(), node.status.node_info.os_image),
        (
            "Architecture".to_string(),
            node.status.node_info.architecture,
        ),
        (
            "Container Runtime".to_string(),
            node.status.node_info.container_runtime_version,
        ),
        ("Kubelet".to_string(), node.status.node_info.kubelet_version),
        (
            "Kube Proxy".to_string(),
            node.status.node_info.kube_proxy_version,
        ),
    ];

    resource_info_view(&data)
        .attr("id", "node-info")
        .attr(
            "hx-get",
            format!(
                "{}/cluster/nodes/{node_name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}
