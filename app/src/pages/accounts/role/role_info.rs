use api::accounts::roles as roles_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn RoleInfoComponent(
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
        let role = roles_api::get_roles(namespace_name)
            .await
            .unwrap_or_default()
            .iter()
            .find(|sc| sc.metadata.name == resource_name)
            .cloned()
            .unwrap_or_default();

        data.set(
            vec![
                ("Name", role.clone().metadata.name),
                ("Kind", "PersistentVolume".to_string()),
                (
                    "Created",
                    format_timestamp(
                        &role.clone().metadata.creation_timestamp.unwrap_or_default(),
                        None,
                    ),
                ),
                ("Labels", display::hashmap(role.clone().metadata.labels)),
                (
                    "Annotations",
                    display::hashmap(role.clone().metadata.annotations),
                ),
                ("Version", role.metadata.resource_version),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
