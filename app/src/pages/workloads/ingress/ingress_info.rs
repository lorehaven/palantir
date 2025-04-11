use leptos::prelude::*;
use leptos::task::spawn_local;

use api::workloads::ingresses as ingresses_api;
use crate::components::shared::info::resource_info_view;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

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

    resource_info_view(ingress_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    ingress_name: RwSignal<String>,
    ingress_data: RwSignal<Vec<(String, String)>>,
) {
    if namespace_name.is_disposed() || ingress_name.is_disposed() { return; }
    let selected_value = namespace_name.get();
    let ingress_name = ingress_name.get();

    spawn_local(async move {
        let selected_value = if selected_value == "All Namespaces" { None } else { Some(selected_value) };
        let ingress = ingresses_api::get_ingresses(selected_value).await.unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == ingress_name)
            .unwrap_or_default();

        ingress_data.set(vec![
            ("Name", ingress.clone().metadata.name),
            ("Kind", "Ingress".to_string()),
            ("Namespace", ingress.clone().metadata.namespace),
            ("Created", format_timestamp(&ingress.clone().metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Annotations", display::hashmap(ingress.clone().metadata.annotations)),
            ("Version", ingress.metadata.resource_version),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
