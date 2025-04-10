use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::accounts::roles as roles_api;
use crate::components::shared::info::resource_info_view;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

#[component]
pub fn ClusterRoleInfoComponent(
    cluster_role_name: String,
) -> impl IntoView {
    let cluster_role_name = RwSignal::new(cluster_role_name);
    let cluster_role_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(cluster_role_name, cluster_role_data));
    clear_page_effect(interval_handle);

    resource_info_view(cluster_role_data)
}

fn update_page(
    cluster_role_name: RwSignal<String>,
    cluster_role_data: RwSignal<Vec<(String, String)>>,
) {
    if cluster_role_name.is_disposed() { return; }
    let cluster_role_name = cluster_role_name.get();

    spawn_local(async move {
        let cr = roles_api::get_clusterroles().await
            .unwrap_or_default()
            .iter().find(|sc| sc.metadata.name == cluster_role_name)
            .cloned()
            .unwrap_or_default();

        cluster_role_data.set(vec![
            ("Name", cr.clone().metadata.name),
            ("Kind", "PersistentVolume".to_string()),
            ("Created", format_timestamp(&cr.clone().metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Labels", cr.clone().metadata.labels.into_iter()
                .map(|(k, v)| format!("{k} • {v}"))
                .collect::<Vec<String>>()
                .join("\n")),
            ("Annotations", cr.clone().metadata.annotations.into_iter()
                .map(|(k, v)| format!("{k} • {v}"))
                .collect::<Vec<String>>()
                .join("\n")),
            ("Version", cr.metadata.resource_version),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
