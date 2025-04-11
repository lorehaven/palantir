use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;
use api::accounts::bindings as bindings_api;

#[component]
pub fn RoleBindingInfoComponent(
    namespace_name: String,
    binding_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let binding_name = RwSignal::new(binding_name);
    let binding_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(namespace_name, binding_name, binding_data));
    clear_page_effect(interval_handle);

    resource_info_view(binding_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    binding_name: RwSignal<String>,
    binding_data: RwSignal<Vec<(String, String)>>,
) {
    if binding_name.is_disposed() || namespace_name.is_disposed() { return; }
    let namespace_name = namespace_name.get();
    let binding_name = binding_name.get();

    spawn_local(async move {
        let selected_value = if namespace_name == "All Namespaces" { None } else { Some(namespace_name) };
        let binding = bindings_api::get_rolebindings(selected_value).await
            .unwrap_or_default()
            .iter().find(|sc| sc.metadata.name == binding_name)
            .cloned()
            .unwrap_or_default();

        binding_data.set(vec![
            ("Name", binding.clone().metadata.name),
            ("Kind", "PersistentVolume".to_string()),
            ("Namespace", binding.clone().metadata.namespace),
            ("Created", format_timestamp(&binding.clone().metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Labels", display::hashmap(binding.clone().metadata.labels)),
            ("Annotations", display::hashmap(binding.clone().metadata.annotations)),
            ("Version", binding.metadata.resource_version),
            ("Role", binding.role_ref.name),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
