use api::workloads::replicasets as replicasets_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn DeploymentReplicaSetsComponent(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let table_rows = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Generations", TableColumnType::String, 3),
        TableColumn::new("Replicas", TableColumnType::String, 2),
        // TableColumn::new("Active", TableColumnType::Switch, 2),
    ];
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = format!("/workloads/{}/replicasets/", namespace_name.get_untracked());

    let columns_update = columns.clone();
    let interval_handle = update_page_effect(10_000, move || {
        update_page(
            columns_update.clone(),
            styles.clone(),
            params.clone(),
            table_rows,
            namespace_name,
            resource_name,
            loading,
        );
    });
    clear_page_effect(interval_handle);
    data_list_view(columns, table_rows, loading)
}

fn update_page(
    columns: Vec<TableColumn>,
    styles: Vec<String>,
    params: Vec<String>,
    table_rows: RwSignal<Vec<TableRow>>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    loading: RwSignal<bool>,
) {
    if namespace_name.is_disposed() || resource_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let resource_name = resource_name.get();

    spawn_local(async move {
        let list = update_page_async(
            columns.clone(),
            styles.clone(),
            params.clone(),
            namespace_name.clone(),
            resource_name.clone(),
        )
        .await
        .unwrap_or_default();
        table_rows.set(list);
        loading.set(false);
    });
}

#[server]
async fn update_page_async(
    columns: Vec<TableColumn>,
    styles: Vec<String>,
    params: Vec<String>,
    namespace_name: String,
    resource_name: String,
) -> Result<Vec<TableRow>, ServerFnError> {
    let namespace_name = if namespace_name == "All Namespaces" {
        None
    } else {
        Some(namespace_name)
    };
    let mut list = replicasets_api::get_replicasets(namespace_name)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|p| p.metadata.name.contains(&resource_name))
        .map(|r| {
            vec![
                "ReplicaSet".to_string(),
                r.clone().metadata.name,
                r.metadata.generation.to_string(),
                format!("{}/{}", r.status.available_replicas, r.status.replicas),
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    Ok(parse_table_rows(columns, list, styles, params))
}
