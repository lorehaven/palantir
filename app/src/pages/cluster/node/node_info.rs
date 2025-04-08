use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::cluster::nodes as nodes_api;
use crate::components::shared::info::resource_info_view;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn NodeInfoComponent(
    node_name: String,
) -> impl IntoView {
    let node_name = RwSignal::new(node_name);
    let node_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(node_name, node_data));
    clear_page_effect(interval_handle);

    resource_info_view(node_data)
}

fn update_page(
    node_name: RwSignal<String>,
    node_data: RwSignal<Vec<(String, String)>>,
) {
    if node_name.is_disposed() { return; }
    let node_name = node_name.get();

    spawn_local(async move {
        let node = nodes_api::get_nodes_response().await
            .unwrap_or_default();
        let kind = if node.kind == "NodesList" { "Node".to_string() } else { node.kind };
        let resource_version = node.metadata.resource_version;
        let node = node.items.into_iter()
            .find(|n| n.metadata.name == node_name)
            .unwrap_or_default();

        let mut items = vec![];
        items.push(("Name", node.metadata.name));
        items.push(("Kind", kind));
        items.push(("Created", node.metadata.creation_timestamp.unwrap_or_default()));
        items.push(("Labels", node.metadata.labels.into_iter()
            .map(|(k, v)| format!("{k} • {v}"))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Annotations", node.metadata.annotations.into_iter()
            .map(|(k, v)| format!("{k} • {v}"))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Version", resource_version));
        items.push(("Kernel Version", node.status.node_info.kernel_version));
        items.push(("OS", node.status.node_info.os_image));
        items.push(("Architecture", node.status.node_info.architecture));
        items.push(("Container Runtime", node.status.node_info.container_runtime_version));
        items.push(("Kubelet", node.status.node_info.kubelet_version));
        items.push(("Kube Proxy", node.status.node_info.kube_proxy_version));
        node_data.set(items.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
