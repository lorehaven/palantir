use api::workloads::replicasets as replicasets_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn ReplicaSetInfoComponent(
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
        let replicaset = replicasets_api::get_replicasets(None)
            .await
            .unwrap_or_default();
        let replicaset = replicaset
            .into_iter()
            .find(|n| n.metadata.namespace == namespace_name && n.metadata.name == resource_name)
            .unwrap_or_default();

        data.set(
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
                            format!("{}/{namespace_name}/{}", or.kind.to_lowercase(), or.name)
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
