use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::cluster::namespaces as namespaces_api;
use crate::components::shared::info::resource_info_view;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

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
    spawn_local(async move {
        if namespace_name.is_disposed() { return; }

        let namespace = namespaces_api::get_namespaces_response().await
            .unwrap_or_default();
        let kind = if namespace.kind == "NamespaceList" { "Namespace".to_string() } else { namespace.kind };
        let namespace = namespace.items.into_iter()
            .find(|n| n.metadata.name == namespace_name.get_untracked())
            .unwrap_or_default();

        let mut items = vec![];
        items.push(("Name", namespace.metadata.name));
        items.push(("Kind", kind));
        items.push(("Created", format_timestamp(&namespace.metadata.creation_timestamp.unwrap_or_default(), None)));
        items.push(("Labels", namespace.metadata.labels.into_iter()
            .map(|(k, v)| format!("{k} • {v}"))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Annotations", namespace.metadata.annotations.into_iter()
            .map(|(k, v)| format!("{k} • {v}"))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Version", namespace.metadata.resource_version));
        items.push(("Status", namespace.status.phase));
        namespace_data.set(items.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
