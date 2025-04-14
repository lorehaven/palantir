use api::accounts::bindings as bindings_api;
use domain::utils::time::time_until_now;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::pages::accounts::update_page_list_async;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn RoleBindingSubjectsComponent(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let table_rows = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 3),
        TableColumn::new("Namespace", TableColumnType::String, 3),
        TableColumn::new("Name", TableColumnType::String, 3),
        TableColumn::new("Api Group", TableColumnType::String, 3),
    ];
    let styles = vec![String::new(); columns.len()];
    let params = vec![String::new(); columns.len()];

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
    if resource_name.is_disposed() || namespace_name.is_disposed() {
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
    let mut list = bindings_api::get_rolebindings(namespace_name)
        .await
        .unwrap_or_default()
        .iter()
        .find(|sc| sc.metadata.name == resource_name)
        .cloned()
        .unwrap_or_default()
        .subjects
        .into_iter()
        .map(|r| vec![r.kind, r.namespace, r.name, r.api_group])
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    Ok(parse_table_rows(columns, list, styles, params))
}
