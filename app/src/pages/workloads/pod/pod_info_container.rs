use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::pods as pods_api;
use crate::components::shared::info::resource_info_view;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn PodInfoContainerComponent(
    namespace_name: String,
    pod_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let pod_name = RwSignal::new(pod_name);
    let pod_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(
        namespace_name,
        pod_name,
        pod_data,
    ));
    clear_page_effect(interval_handle);

    resource_info_view(pod_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    pod_name: RwSignal<String>,
    pod_data: RwSignal<Vec<(String, String)>>,
) {
    spawn_local(async move {
        if namespace_name.is_disposed() || pod_name.is_disposed() { return; }

        let namespace_name = namespace_name.get_untracked();
        let pod_name = pod_name.get_untracked();
        let pod = pods_api::get_pods_by_namespace_name(namespace_name.clone()).await.unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == pod_name)
            .unwrap_or_default();
        let container = pod.spec.containers.first().cloned().unwrap_or_default();

        let mut items = vec![];
        items.push(("Container", container.name));
        items.push(("Image", container.image));
        items.push(("Env", container.env.into_iter()
            .map(|e| format!("{}: {}", e.name, e.value))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Cpu Request", container.resources.requests.cpu));
        items.push(("Memory Request", container.resources.requests.memory));
        items.push(("Cpu Limit", container.resources.limits.cpu));
        items.push(("Memory Limit", container.resources.limits.memory));
        items.push(("Ports", container.ports.into_iter()
            .map(|p| {
                let name = if p.name.is_empty() { String::new() } else { format!("{} • ", p.name) };
                format!("{name}{} • {}", p.container_port, p.protocol)
            })
            .collect::<Vec<String>>()
            .join("\n")));
        pod_data.set(items.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
