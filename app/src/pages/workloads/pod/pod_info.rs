use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::pods as pods_api;
use crate::components::shared::info::resource_info_view;
use crate::pages::utils::shared::display;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

#[component]
pub fn PodInfoComponent(
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
        let pod = pods_api::get_pods_by_namespace_name(selected_value.clone()).await.unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == pod_name)
            .unwrap_or_default();

        pod_data.set(vec![
            ("Name", pod.clone().metadata.name),
            ("Kind", "Pod".to_string()),
            ("Namespace", pod.clone().metadata.namespace),
            ("Created", format_timestamp(&pod.clone().metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Labels", display::hashmap(pod.clone().metadata.labels)),
            ("Annotations", display::hashmap(pod.clone().metadata.annotations)),
            ("Version", pod.clone().metadata.resource_version),
            ("Owned By", pod.clone().metadata.owner_references.into_iter()
                .map(|or| format!("{}/{selected_value}/{}", or.kind.to_lowercase(), or.name))
                .collect::<Vec<String>>()
                .join("\n")),
            ("Host IP", pod.clone().status.host_ip),
            ("Pod IP", pod.clone().status.pod_ip.unwrap_or_default()),
            ("QOS", pod.clone().status.qos_class),
            ("Phase", pod.clone().status.phase),
            ("Conditions", pod.clone().status.conditions.into_iter()
                .map(|c| format!("{} • {}", c.r#type, c.status))
                .collect::<Vec<String>>()
                .join("\n")),
            ("Node Name", pod.clone().spec.node_name),
            ("Selector", display::hashmap(pod.spec.selector)),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
