use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use api::workloads::pods as pods_api;

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
    if namespace_name.is_disposed() || pod_name.is_disposed() { return; }
    let selected_value = namespace_name.get();
    let pod_name = pod_name.get();

    spawn_local(async move {
        let namespace_name = if selected_value.clone() == "All Namespaces" { None } else { Some(selected_value.clone()) };
        let pod = pods_api::get_pods(namespace_name, None).await.unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == pod_name)
            .unwrap_or_default();
        let container = pod.spec.containers.first().cloned().unwrap_or_default();

        pod_data.set(vec![
            ("Container", container.name),
            ("Image", container.image),
            ("Env", container.env.into_iter()
                .map(|e| format!("{}: {}", e.name, e.value))
                .collect::<Vec<String>>()
                .join("\n")),
            ("Cpu Request", container.resources.requests.cpu),
            ("Memory Request", container.resources.requests.memory),
            ("Cpu Limit", container.resources.limits.cpu),
            ("Memory Limit", container.resources.limits.memory),
            ("Ports", container.ports.into_iter()
                .map(|p| {
                    let name = if p.name.is_empty() { String::new() } else { format!("{} • ", p.name) };
                    format!("{name}{} • {}", p.container_port, p.protocol)
                })
                .collect::<Vec<String>>()
                .join("\n")),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
