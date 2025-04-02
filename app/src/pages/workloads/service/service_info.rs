use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::services as services_api;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

#[component]
pub fn ServiceInfoComponent(
    namespace_name: String,
    service_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let service_name = RwSignal::new(service_name);
    let service_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(
        namespace_name,
        service_name,
        service_data,
    ));
    clear_page_effect(interval_handle);

    view(service_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    service_name: RwSignal<String>,
    service_data: RwSignal<Vec<(&'static str, String)>>,
) {
    spawn_local(async move {
        if namespace_name.is_disposed() || service_name.is_disposed() { return; }

        let service = services_api::get_services(None).await
            .unwrap_or_default();
        let service = service.into_iter()
            .find(|n| n.metadata.namespace == namespace_name.get_untracked() && n.metadata.name == service_name.get_untracked())
            .unwrap_or_default();

        let mut items = vec![];
        items.push(("Name", service.metadata.name));
        items.push(("Kind", "Service".to_string()));
        items.push(("Namespace", service.metadata.namespace));
        items.push(("Created", format_timestamp(&service.metadata.creation_timestamp, None)));
        items.push(("Labels", service.metadata.labels.into_iter()
            .map(|(k, v)| format!("{} • {}", k, v))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Annotations", service.metadata.annotations.into_iter()
            .map(|(k, v)| format!("{} • {}", k, v))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Version", service.metadata.resource_version));
        items.push(("Cluster IP", service.spec.cluster_ip));
        items.push(("Type", service.spec.r#type));
        items.push(("Affinity", service.spec.session_affinity));
        items.push(("Selector", service.spec.selector.into_iter()
            .map(|(k, v)| format!("{} • {}", k, v))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Ports", service.spec.ports.into_iter()
            .map(|p| {
                let target_port = match p.target_port {
                    Some(ref tp) => format!(" • {tp}"),
                    None => "".to_string(),
                };
                let node_port = match p.node_port {
                    Some(ref tp) => format!(" • {tp}"),
                    None => "".to_string(),
                };
                format!("{} • {}{target_port}{node_port} • {}", p.name, p.port.unwrap_or(0), p.protocol)
            })
            .collect::<Vec<String>>()
            .join("\n")));
        service_data.set(items);
    });
}

fn view(
    service_data: RwSignal<Vec<(&'static str, String)>>,
) -> impl IntoView {
    view! {
        <div class="card-container dcc-1">
            <div class="card-list">
                {move || service_data.get().into_iter().map(|(name, value)| view! {
                    <div class="card-list-row">
                        <div class="card-list-row-title">{name}</div>
                        <div class="card-list-row-content">{value}</div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
