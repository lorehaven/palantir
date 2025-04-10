use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::accounts::roles as roles_api;
use crate::components::shared::info::resource_info_view;
use crate::pages::utils::shared::display;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

#[component]
pub fn RoleInfoComponent(
    namespace_name: String,
    role_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let role_name = RwSignal::new(role_name);
    let role_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(namespace_name, role_name, role_data));
    clear_page_effect(interval_handle);

    resource_info_view(role_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    role_name: RwSignal<String>,
    role_data: RwSignal<Vec<(String, String)>>,
) {
    if role_name.is_disposed() || namespace_name.is_disposed() { return; }
    let namespace_name = namespace_name.get();
    let role_name = role_name.get();

    spawn_local(async move {
        let selected_value = if namespace_name == "All Namespaces" { None } else { Some(namespace_name) };
        let role = roles_api::get_roles(selected_value).await
            .unwrap_or_default()
            .iter().find(|sc| sc.metadata.name == role_name)
            .cloned()
            .unwrap_or_default();

        role_data.set(vec![
            ("Name", role.clone().metadata.name),
            ("Kind", "PersistentVolume".to_string()),
            ("Created", format_timestamp(&role.clone().metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Labels", display::hashmap(role.clone().metadata.labels)),
            ("Annotations", display::hashmap(role.clone().metadata.annotations)),
            ("Version", role.metadata.resource_version),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
