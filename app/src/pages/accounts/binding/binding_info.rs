use api::accounts::bindings as bindings_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn RoleBindingInfoComponent(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let binding_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || {
        update_page(namespace_name, resource_name, binding_data);
    });
    clear_page_effect(interval_handle);

    resource_info_view(binding_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    binding_data: RwSignal<Vec<(String, String)>>,
) {
    if resource_name.is_disposed() || namespace_name.is_disposed() {
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
        let binding = bindings_api::get_rolebindings(namespace_name)
            .await
            .unwrap_or_default()
            .iter()
            .find(|sc| sc.metadata.name == resource_name)
            .cloned()
            .unwrap_or_default();

        binding_data.set(
            vec![
                ("Name", binding.clone().metadata.name),
                ("Kind", "PersistentVolume".to_string()),
                ("Namespace", binding.clone().metadata.namespace),
                (
                    "Created",
                    format_timestamp(
                        &binding
                            .clone()
                            .metadata
                            .creation_timestamp
                            .unwrap_or_default(),
                        None,
                    ),
                ),
                ("Labels", display::hashmap(binding.clone().metadata.labels)),
                (
                    "Annotations",
                    display::hashmap(binding.clone().metadata.annotations),
                ),
                ("Version", binding.metadata.resource_version),
                ("Role", binding.role_ref.name),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
