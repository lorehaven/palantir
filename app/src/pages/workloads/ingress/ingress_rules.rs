use api::workloads::ingresses as ingresses_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn IngressRulesComponent(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let table_rows = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let columns = vec![
        TableColumn::new("Host", TableColumnType::String, 2),
        TableColumn::new("Path", TableColumnType::StringList, 2),
        TableColumn::new("Service Name", TableColumnType::String, 2),
        TableColumn::new("Service Port", TableColumnType::String, 2),
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
    let mut list = ingresses_api::get_ingresses(namespace_name)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|n| n.metadata.name == resource_name)
        .unwrap_or_default()
        .spec
        .rules
        .into_iter()
        .map(|rule| {
            let paths = rule
                .clone()
                .http
                .paths
                .into_iter()
                .map(|p| p.path)
                .collect::<Vec<_>>()
                .join("\n");
            let service_names = rule
                .clone()
                .http
                .paths
                .into_iter()
                .map(|p| p.backend.service.name)
                .collect::<Vec<_>>()
                .join("\n");
            let service_ports = rule
                .clone()
                .http
                .paths
                .into_iter()
                .map(|p| p.backend.service.port.number.to_string())
                .collect::<Vec<_>>()
                .join("\n");

            vec![rule.host, paths, service_names, service_ports]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    Ok(parse_table_rows(columns, list, styles, params))
}
