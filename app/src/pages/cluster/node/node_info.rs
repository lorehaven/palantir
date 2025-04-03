use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::cluster::nodes as nodes_api;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn NodeInfoComponent(
    node_name: String,
) -> impl IntoView {
    let node_name = RwSignal::new(node_name);
    let node_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(node_name, node_data));
    clear_page_effect(interval_handle);

    view(node_data)
}

fn update_page(
    node_name: RwSignal<String>,
    node_data: RwSignal<Vec<(&'static str, String)>>,
) {
    spawn_local(async move {
        if node_name.is_disposed() { return; }

        let node = nodes_api::get_nodes_response().await
            .unwrap_or_default();
        let kind = if node.kind == "NodesList" { "Node".to_string() } else { node.kind };
        let resource_version = node.metadata.resource_version;
        let node = node.items.into_iter()
            .find(|n| n.metadata.name == node_name.get_untracked())
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
        node_data.set(items);
    });
}

fn view(
    node_data: RwSignal<Vec<(&'static str, String)>>,
) -> impl IntoView {
    view! {
        <div class="card-container dcc-1">
            <div class="card-list">
                {move || node_data.get().into_iter().map(|(name, value)| view! {
                    <div class="card-list-row">
                        <div class="card-list-row-title">{name}</div>
                        <div class="card-list-row-content">{value}</div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
