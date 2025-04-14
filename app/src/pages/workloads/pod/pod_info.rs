use api::workloads::pods as pods_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn PodInfoComponent(
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
        let pod = pods_api::get_pods(namespace_name.clone(), None)
            .await
            .unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == resource_name)
            .unwrap_or_default();

        data.set(
            vec![
                ("Name", pod.clone().metadata.name),
                ("Kind", "Pod".to_string()),
                ("Namespace", pod.clone().metadata.namespace),
                (
                    "Created",
                    format_timestamp(
                        &pod.clone().metadata.creation_timestamp.unwrap_or_default(),
                        None,
                    ),
                ),
                ("Labels", display::hashmap(pod.clone().metadata.labels)),
                (
                    "Annotations",
                    display::hashmap(pod.clone().metadata.annotations),
                ),
                ("Version", pod.clone().metadata.resource_version),
                (
                    "Owned By",
                    pod.clone()
                        .metadata
                        .owner_references
                        .into_iter()
                        .map(|or| {
                            format!(
                                "{}/{}/{}",
                                namespace_name.clone().unwrap_or_default(),
                                or.kind.to_lowercase(),
                                or.name
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("\n"),
                ),
                ("Host IP", pod.clone().status.host_ip),
                ("Pod IP", pod.clone().status.pod_ip.unwrap_or_default()),
                ("QOS", pod.clone().status.qos_class),
                ("Phase", pod.clone().status.phase),
                (
                    "Conditions",
                    pod.clone()
                        .status
                        .conditions
                        .into_iter()
                        .map(|c| format!("{} • {}", c.r#type, c.status))
                        .collect::<Vec<String>>()
                        .join("\n"),
                ),
                ("Node Name", pod.clone().spec.node_name),
                ("Selector", display::hashmap(pod.spec.selector)),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
