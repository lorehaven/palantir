use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;
use api::cluster::namespaces as namespaces_api;

#[component]
pub fn NamespaceInfoComponent(
    namespace_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let namespace_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(namespace_name, namespace_data));
    clear_page_effect(interval_handle);

    resource_info_view(namespace_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    namespace_data: RwSignal<Vec<(String, String)>>,
) {
    if namespace_name.is_disposed() { return; }
    let selected_value = namespace_name.get();

    spawn_local(async move {
        let namespace = namespaces_api::get_namespaces_response().await
            .unwrap_or_default();
        let kind = if namespace.kind == "NamespaceList" { "Namespace".to_string() } else { namespace.kind };
        let namespace = namespace.items.into_iter()
            .find(|n| n.metadata.name == selected_value)
            .unwrap_or_default();

        namespace_data.set(vec![
            ("Name", namespace.clone().metadata.name),
            ("Kind", kind),
            ("Created", format_timestamp(&namespace.clone().metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Labels", display::hashmap(namespace.clone().metadata.labels)),
            ("Annotations", display::hashmap(namespace.clone().metadata.annotations)),
            ("Version", namespace.clone().metadata.resource_version),
            ("Status", namespace.status.phase),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
