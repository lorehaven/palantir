use api::workloads::replicasets as replicasets_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn ReplicaSetInfoComponent(namespace_name: String, replicaset_name: String) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let replicaset_name = RwSignal::new(replicaset_name);
    let replicaset_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || {
        update_page(namespace_name, replicaset_name, replicaset_data);
    });
    clear_page_effect(interval_handle);

    resource_info_view(replicaset_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    replicaset_name: RwSignal<String>,
    replicaset_data: RwSignal<Vec<(String, String)>>,
) {
    if namespace_name.is_disposed() || replicaset_name.is_disposed() {
        return;
    }
    let selected_value = namespace_name.get();
    let replicaset_name = replicaset_name.get();

    spawn_local(async move {
        let replicaset = replicasets_api::get_replicasets(None)
            .await
            .unwrap_or_default();
        let replicaset = replicaset
            .into_iter()
            .find(|n| n.metadata.namespace == selected_value && n.metadata.name == replicaset_name)
            .unwrap_or_default();

        replicaset_data.set(
            vec![
                ("Name", replicaset.clone().metadata.name),
                ("Kind", "ReplicaSet".to_string()),
                ("Namespace", replicaset.clone().metadata.namespace),
                (
                    "Created",
                    format_timestamp(
                        &replicaset
                            .clone()
                            .metadata
                            .creation_timestamp
                            .unwrap_or_default(),
                        None,
                    ),
                ),
                (
                    "Labels",
                    display::hashmap(replicaset.clone().metadata.labels),
                ),
                (
                    "Annotations",
                    display::hashmap(replicaset.clone().metadata.annotations),
                ),
                ("Version", replicaset.clone().metadata.resource_version),
                (
                    "Owned By",
                    replicaset
                        .metadata
                        .owner_references
                        .into_iter()
                        .map(|or| {
                            format!("{}/{selected_value}/{}", or.kind.to_lowercase(), or.name)
                        })
                        .collect::<Vec<String>>()
                        .join("\n"),
                ),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
