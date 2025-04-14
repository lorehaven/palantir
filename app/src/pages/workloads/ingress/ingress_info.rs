use api::workloads::ingresses as ingresses_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn IngressInfoComponent(
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
        let namespace_name = if namespace_name == "All Namespaces" {
            None
        } else {
            Some(namespace_name)
        };
        let ingress = ingresses_api::get_ingresses(namespace_name)
            .await
            .unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == resource_name)
            .unwrap_or_default();

        data.set(
            vec![
                ("Name", ingress.clone().metadata.name),
                ("Kind", "Ingress".to_string()),
                ("Namespace", ingress.clone().metadata.namespace),
                (
                    "Created",
                    format_timestamp(
                        &ingress
                            .clone()
                            .metadata
                            .creation_timestamp
                            .unwrap_or_default(),
                        None,
                    ),
                ),
                (
                    "Annotations",
                    display::hashmap(ingress.clone().metadata.annotations),
                ),
                ("Version", ingress.metadata.resource_version),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
