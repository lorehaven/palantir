use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::replicasets as replicasets_api;
use crate::components::shared::info::resource_info_view;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

#[component]
pub fn ReplicaSetInfoComponent(
    namespace_name: String,
    replicaset_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let replicaset_name = RwSignal::new(replicaset_name);
    let replicaset_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(
        namespace_name,
        replicaset_name,
        replicaset_data,
    ));
    clear_page_effect(interval_handle);

    resource_info_view(replicaset_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    replicaset_name: RwSignal<String>,
    replicaset_data: RwSignal<Vec<(String, String)>>,
) {
    spawn_local(async move {
        if namespace_name.is_disposed() || replicaset_name.is_disposed() { return; }

        let namespace_name = namespace_name.get_untracked();
        let replicaset_name = replicaset_name.get_untracked();
        let replicaset = replicasets_api::get_replicasets(None).await
            .unwrap_or_default();
        let replicaset = replicaset.into_iter()
            .find(|n| n.metadata.namespace == namespace_name && n.metadata.name == replicaset_name)
            .unwrap_or_default();

        let mut items = vec![];
        items.push(("Name", replicaset.metadata.name));
        items.push(("Kind", "ReplicaSet".to_string()));
        items.push(("Namespace", replicaset.metadata.namespace));
        items.push(("Created", format_timestamp(&replicaset.metadata.creation_timestamp.unwrap_or_default(), None)));
        items.push(("Labels", replicaset.metadata.labels.into_iter()
            .map(|(k, v)| format!("{k} • {v}"))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Annotations", replicaset.metadata.annotations.into_iter()
            .map(|(k, v)| format!("{k} • {v}"))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Version", replicaset.metadata.resource_version));
        items.push(("Owned By", replicaset.metadata.owner_references.into_iter()
            .map(|or| format!("{}/{namespace_name}/{}", or.kind.to_lowercase(), or.name))
            .collect::<Vec<String>>()
            .join("\n")));
        replicaset_data.set(items.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
