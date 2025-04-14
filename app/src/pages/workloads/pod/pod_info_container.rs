use api::workloads::pods as pods_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn PodInfoContainerComponent(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || {
        update_page(namespace_name, resource_name, data);
    });
    clear_page_effect(interval_handle);

    resource_info_view(data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    data: RwSignal<Vec<(String, String)>>,
) {
    if namespace_name.is_disposed() || resource_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let resource_name = resource_name.get();

    spawn_local(async move {
        let namespace_name = if namespace_name.clone() == "All Namespaces" {
            None
        } else {
            Some(namespace_name.clone())
        };
        let pod = pods_api::get_pods(namespace_name, None)
            .await
            .unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == resource_name)
            .unwrap_or_default();
        let container = pod.spec.containers.first().cloned().unwrap_or_default();

        data.set(
            vec![
                ("Container", container.name),
                ("Image", container.image),
                (
                    "Env",
                    container
                        .env
                        .into_iter()
                        .map(|e| format!("{}: {}", e.name, e.value))
                        .collect::<Vec<String>>()
                        .join("\n"),
                ),
                ("Cpu Request", container.resources.requests.cpu),
                ("Memory Request", container.resources.requests.memory),
                ("Cpu Limit", container.resources.limits.cpu),
                ("Memory Limit", container.resources.limits.memory),
                (
                    "Ports",
                    container
                        .ports
                        .into_iter()
                        .map(|p| {
                            let name = if p.name.is_empty() {
                                String::new()
                            } else {
                                format!("{} • ", p.name)
                            };
                            format!("{name}{} • {}", p.container_port, p.protocol)
                        })
                        .collect::<Vec<String>>()
                        .join("\n"),
                ),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
