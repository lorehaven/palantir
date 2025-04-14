use api::cluster::namespaces as namespaces_api;
use domain::utils::time::time_until_now;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn NamespacesListComponent(resource_name: RwSignal<String>) -> impl IntoView {
    let table_rows = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::Link, 2),
        TableColumn::new("Age", TableColumnType::String, 1),
        TableColumn::new("Status", TableColumnType::String, 1),
    ];
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/cluster/namespaces/".to_string();

    let columns_update = columns.clone();
    let interval_handle = update_page_effect(10_000, move || {
        update_page(
            columns_update.clone(),
            styles.clone(),
            params.clone(),
            table_rows,
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
    resource_name: RwSignal<String>,
    loading: RwSignal<bool>,
) {
    if resource_name.is_disposed() {
        return;
    }
    let resource_name = resource_name.get();

    spawn_local(async move {
        let list = update_page_async(
            columns.clone(),
            styles.clone(),
            params.clone(),
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
    resource_name: String,
) -> Result<Vec<TableRow>, ServerFnError> {
    let mut list = namespaces_api::get_namespaces()
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|n| {
            n.metadata
                .name
                .to_lowercase()
                .contains(&resource_name.to_lowercase())
        })
        .map(|n| {
            vec![
                "Namespace".to_string(),
                n.clone().metadata.name,
                time_until_now(&n.clone().metadata.creation_timestamp.unwrap_or_default()),
                n.status.phase,
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    Ok(parse_table_rows(columns, list, styles, params))
}
