use leptos::prelude::*;
use leptos::task::spawn_local;

use api::accounts::bindings as bindings_api;
use crate::components::shared::info::resource_info_view;
use crate::pages::utils::shared::display;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

#[component]
pub fn ClusterRoleBindingInfoComponent(
    cluster_binding_name: String,
) -> impl IntoView {
    let cluster_binding_name = RwSignal::new(cluster_binding_name);
    let cluster_binding_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(cluster_binding_name, cluster_binding_data));
    clear_page_effect(interval_handle);

    resource_info_view(cluster_binding_data)
}

fn update_page(
    cluster_binding_name: RwSignal<String>,
    cluster_binding_data: RwSignal<Vec<(String, String)>>,
) {
    if cluster_binding_name.is_disposed() { return; }
    let cluster_binding_name = cluster_binding_name.get();

    spawn_local(async move {
        let crb = bindings_api::get_clusterrolebindings().await
            .unwrap_or_default()
            .iter().find(|sc| sc.metadata.name == cluster_binding_name)
            .cloned()
            .unwrap_or_default();

        cluster_binding_data.set(vec![
            ("Name", crb.clone().metadata.name),
            ("Kind", "PersistentVolume".to_string()),
            ("Created", format_timestamp(&crb.clone().metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Labels", display::hashmap(crb.clone().metadata.labels)),
            ("Annotations", display::hashmap(crb.clone().metadata.annotations)),
            ("Version", crb.metadata.resource_version),
            ("Role", crb.role_ref.name),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
