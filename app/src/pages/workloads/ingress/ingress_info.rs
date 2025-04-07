use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::ingresses as ingresses_api;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

#[component]
pub fn IngressInfoComponent(
    namespace_name: String,
    ingress_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let ingress_name = RwSignal::new(ingress_name);
    let ingress_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(
        namespace_name,
        ingress_name,
        ingress_data,
    ));
    clear_page_effect(interval_handle);

    view(ingress_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    ingress_name: RwSignal<String>,
    ingress_data: RwSignal<Vec<(&'static str, String)>>,
) {
    spawn_local(async move {
        if namespace_name.is_disposed() || ingress_name.is_disposed() { return; }

        let namespace_name = namespace_name.get_untracked();
        let ingress_name = ingress_name.get_untracked();
        let selected_value = if namespace_name == "All Namespaces" { None } else { Some(namespace_name) };
        let ingress = ingresses_api::get_ingresses(selected_value).await.unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == ingress_name)
            .unwrap_or_default();

        let mut items = vec![];
        items.push(("Name", ingress.metadata.name));
        items.push(("Kind", "Ingress".to_string()));
        items.push(("Namespace", ingress.metadata.namespace));
        items.push(("Created", format_timestamp(&ingress.metadata.creation_timestamp.unwrap_or_default(), None)));
        items.push(("Annotations", ingress.metadata.annotations.into_iter()
            .map(|(k, v)| format!("{k} • {v}"))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Version", ingress.metadata.resource_version));
        ingress_data.set(items);
    });
}

fn view(
    ingress_data: RwSignal<Vec<(&'static str, String)>>,
) -> impl IntoView {
    view! {
        <div class="card-container dcc-1">
            <div class="card-list">
                {move || ingress_data.get().into_iter().map(|(name, value)| view! {
                    <div class="card-list-row">
                        <div class="card-list-row-title">{name}</div>
                        <div class="card-list-row-content">{value}</div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
