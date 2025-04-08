use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::pods as pods_api;
use crate::components::shared::info::resource_info_view;
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
    spawn_local(async move {
        if namespace_name.is_disposed() || pod_name.is_disposed() { return; }

        let namespace_name = namespace_name.get_untracked();
        let pod_name = pod_name.get_untracked();
        let pod = pods_api::get_pods_by_namespace_name(namespace_name.clone()).await.unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == pod_name)
            .unwrap_or_default();

        let mut items = vec![];
        items.push(("Name", pod.metadata.name));
        items.push(("Kind", "Pod".to_string()));
        items.push(("Namespace", pod.metadata.namespace));
        items.push(("Created", format_timestamp(&pod.metadata.creation_timestamp.unwrap_or_default(), None)));
        items.push(("Labels", pod.metadata.labels.into_iter()
            .map(|(k, v)| format!("{k} • {v}"))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Version", pod.metadata.resource_version));
        items.push(("Owned By", pod.metadata.owner_references.into_iter()
            .map(|or| format!("{}/{namespace_name}/{}", or.kind.to_lowercase(), or.name))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Host IP", pod.status.host_ip));
        items.push(("Pod IP", pod.status.pod_ip.unwrap_or_default()));
        items.push(("QOS", pod.status.qos_class));
        items.push(("Phase", pod.status.phase));
        items.push(("Conditions", pod.status.conditions.into_iter()
            .map(|c| format!("{} • {}", c.r#type, c.status))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Node Name", pod.spec.node_name));
        items.push(("Selector", pod.spec.selector.into_iter()
            .map(|(k, v)| format!("{k} • {v}"))
            .collect::<Vec<String>>()
            .join("\n")));
        pod_data.set(items.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
