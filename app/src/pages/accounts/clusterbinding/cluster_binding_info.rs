use api::accounts::bindings as bindings_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn ClusterRoleBindingInfoComponent(resource_name: RwSignal<String>) -> impl IntoView {
    let data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || {
        update_page(resource_name, data);
    });
    clear_page_effect(interval_handle);

    resource_info_view(data)
}

fn update_page(resource_name: RwSignal<String>, data: RwSignal<Vec<(String, String)>>) {
    if resource_name.is_disposed() {
        return;
    }
    let resource_name = resource_name.get();

    spawn_local(async move {
        let crb = bindings_api::get_clusterrolebindings()
            .await
            .unwrap_or_default()
            .iter()
            .find(|sc| sc.metadata.name == resource_name)
            .cloned()
            .unwrap_or_default();

        data.set(
            vec![
                ("Name", crb.clone().metadata.name),
                ("Kind", "PersistentVolume".to_string()),
                (
                    "Created",
                    format_timestamp(
                        &crb.clone().metadata.creation_timestamp.unwrap_or_default(),
                        None,
                    ),
                ),
                ("Labels", display::hashmap(crb.clone().metadata.labels)),
                (
                    "Annotations",
                    display::hashmap(crb.clone().metadata.annotations),
                ),
                ("Version", crb.metadata.resource_version),
                ("Role", crb.role_ref.name),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
