use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use api::workloads::replicasets as replicasets_api;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn ReplicaSetsListComponent(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let replicasets = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(selected, prompt, replicasets));
    clear_page_effect(interval_handle);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 2),
        TableColumn::new("Name", TableColumnType::Link, 4),
        TableColumn::new("Generations", TableColumnType::String, 1),
        TableColumn::new("Replicas", TableColumnType::String, 2),
        // TableColumn::new("Active", TableColumnType::Switch, 2),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/workloads/:1/replicasets/";
    data_list_view(columns, replicasets, styles, params)
}

fn update_page(
    namespace_name: RwSignal<String>,
    replicaset_name: RwSignal<String>,
    replicasets: RwSignal<Vec<Vec<String>>>,
) {
    if namespace_name.is_disposed() || replicaset_name.is_disposed() { return; }
    let selected_value = namespace_name.get();
    let replicaset_name = replicaset_name.get();

    spawn_local(async move {
        let selected_value = if selected_value == "All Namespaces" { None } else { Some(selected_value) };
        let replicasets_data = replicasets_api::get_replicasets(selected_value).await.unwrap_or_default();

        replicasets.set(replicasets_data
            .into_iter()
            .filter(|s| s.metadata.name.to_lowercase().contains(&replicaset_name.to_lowercase()))
            .map(|r| vec![
                "ReplicaSet".to_string(),
                r.clone().metadata.namespace,
                r.clone().metadata.name,
                r.metadata.generation.to_string(),
                format!("{}/{}", r.status.available_replicas, r.status.replicas),
            ])
            .collect());
    });
}
