use api::workloads::services as services_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn ServiceInfoComponent(
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
        let service = services_api::get_services(None).await.unwrap_or_default();
        let service = service
            .into_iter()
            .find(|n| n.metadata.namespace == namespace_name && n.metadata.name == resource_name)
            .unwrap_or_default();

        data.set(
            vec![
                ("Name", service.clone().metadata.name),
                ("Kind", "Service".to_string()),
                ("Namespace", service.clone().metadata.namespace),
                (
                    "Created",
                    format_timestamp(
                        &service
                            .clone()
                            .metadata
                            .creation_timestamp
                            .unwrap_or_default(),
                        None,
                    ),
                ),
                ("Labels", display::hashmap(service.clone().metadata.labels)),
                (
                    "Annotations",
                    display::hashmap(service.clone().metadata.annotations),
                ),
                ("Version", service.clone().metadata.resource_version),
                ("Cluster IP", service.clone().spec.cluster_ip),
                ("Type", service.clone().spec.r#type),
                ("Affinity", service.clone().spec.session_affinity),
                ("Selector", display::hashmap(service.clone().spec.selector)),
                (
                    "Ports",
                    service
                        .spec
                        .ports
                        .into_iter()
                        .map(|p| {
                            let target_port = p
                                .target_port
                                .as_ref()
                                .map_or_else(String::new, |tp| format!(" • {tp}"));
                            let node_port = p
                                .node_port
                                .as_ref()
                                .map_or_else(String::new, |tp| format!(" • {tp}"));
                            format!(
                                "{} • {}{target_port}{node_port} • {}",
                                p.name,
                                p.port.unwrap_or(0),
                                p.protocol
                            )
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
