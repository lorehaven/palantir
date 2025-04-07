use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::configmaps as configmaps_api;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

#[component]
pub fn ConfigMapInfoComponent(
    namespace_name: String,
    configmap_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let configmap_name = RwSignal::new(configmap_name);
    let configmap_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(
        namespace_name,
        configmap_name,
        configmap_data,
    ));
    clear_page_effect(interval_handle);

    view(configmap_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    configmap_name: RwSignal<String>,
    configmap_data: RwSignal<Vec<(&'static str, String)>>,
) {
    spawn_local(async move {
        if namespace_name.is_disposed() || configmap_name.is_disposed() { return; }

        let namespace_name = namespace_name.get_untracked();
        let configmap_name = configmap_name.get_untracked();
        let selected_value = if namespace_name == "All Namespaces" { None } else { Some(namespace_name) };
        let configmap = configmaps_api::get_configmaps(selected_value).await.unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == configmap_name)
            .unwrap_or_default();

        configmap_data.set(vec![
            ("Name", configmap.metadata.name),
            ("Kind", "ConfigMap".to_string()),
            ("Namespace", configmap.metadata.namespace),
            ("Created", format_timestamp(&configmap.metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Version", configmap.metadata.resource_version),
        ]);
    });
}

fn view(
    configmap_data: RwSignal<Vec<(&'static str, String)>>,
) -> impl IntoView {
    view! {
        <div class="card-container dcc-1">
            <div class="card-list">
                {move || configmap_data.get().into_iter().map(|(name, value)| view! {
                    <div class="card-list-row">
                        <div class="card-list-row-title">{name}</div>
                        <div class="card-list-row-content">{value}</div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
