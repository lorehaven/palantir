use api::cluster::nodes as nodes_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn NodeInfoComponent(resource_name: RwSignal<String>) -> impl IntoView {
    let data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(resource_name, data));
    clear_page_effect(interval_handle);

    resource_info_view(data)
}

fn update_page(node_name: RwSignal<String>, data: RwSignal<Vec<(String, String)>>) {
    if node_name.is_disposed() {
        return;
    }
    let node_name = node_name.get();

    spawn_local(async move {
        let node = nodes_api::get_nodes_response().await.unwrap_or_default();
        let kind = if node.kind == "NodesList" {
            "Node".to_string()
        } else {
            node.kind
        };
        let node = node
            .items
            .into_iter()
            .find(|n| n.metadata.name == node_name)
            .unwrap_or_default();

        data.set(
            vec![
                ("Name", node.clone().metadata.name),
                ("Kind", kind),
                (
                    "Created",
                    format_timestamp(
                        &node.clone().metadata.creation_timestamp.unwrap_or_default(),
                        None,
                    ),
                ),
                ("Labels", display::hashmap(node.clone().metadata.labels)),
                (
                    "Annotations",
                    display::hashmap(node.clone().metadata.annotations),
                ),
                ("Version", node.clone().metadata.resource_version),
                ("Kernel Version", node.status.node_info.kernel_version),
                ("OS", node.status.node_info.os_image),
                ("Architecture", node.status.node_info.architecture),
                (
                    "Container Runtime",
                    node.status.node_info.container_runtime_version,
                ),
                ("Kubelet", node.status.node_info.kubelet_version),
                ("Kube Proxy", node.status.node_info.kube_proxy_version),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
