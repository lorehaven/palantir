use api::cluster::events as events_api;
use domain::utils::time::time_until_now;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn EventsListComponent(
    object_type: String,
    namespace_name: RwSignal<String>,
    object_name: RwSignal<String>,
) -> impl IntoView {
    let object_type = RwSignal::new(object_type);
    let table_rows = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let columns = vec![
        TableColumn::new("Time", TableColumnType::String, 1),
        TableColumn::new("Reason", TableColumnType::String, 1),
        TableColumn::new("Event", TableColumnType::String, 3),
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
            object_type,
            namespace_name,
            object_name,
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
    object_type: RwSignal<String>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    loading: RwSignal<bool>,
) {
    if object_type.is_disposed() || namespace_name.is_disposed() || resource_name.is_disposed() {
        return;
    }
    let object_type = object_type.get();
    let namespace_name = namespace_name.get();
    let object_name = resource_name.get();

    spawn_local(async move {
        let list = update_page_async(
            columns.clone(),
            styles.clone(),
            params.clone(),
            object_type.clone(),
            namespace_name.clone(),
            object_name.clone(),
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
    resource_type: String,
    namespace_name: String,
    resource_name: String,
) -> Result<Vec<TableRow>, ServerFnError> {
    let namespace_name = if namespace_name == "All Namespaces" {
        None
    } else {
        Some(namespace_name)
    };
    let mut list = events_api::get_events(namespace_name)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|e| {
            e.involved_object.kind == resource_type && e.involved_object.name == resource_name
        })
        .map(|event| {
            vec![
                time_until_now(&event.first_timestamp.unwrap_or_default()),
                event.reason,
                event.message,
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[0].cmp(&b[0]));
    Ok(parse_table_rows(columns, list, styles, params))
}
