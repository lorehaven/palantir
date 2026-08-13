use api::storage::claims as claims_api;
use domain::utils::time::time_until_now;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const FRAGMENT_ID: &str = "claims-table";
const FILTER_ID: &str = "claims-filter";
const NAMESPACE_ID: &str = "claims-namespace";
const POLL_TRIGGER: &str =
    "every 10s, keyup changed delay:300ms from:#claims-filter, change from:#claims-namespace";
const INCLUDE: &str = "#claims-filter, #claims-namespace";

pub async fn render(
    cache: &CacheStore,
    current_path: &str,
    namespace: &str,
    filter: &str,
) -> String {
    let namespace_select = namespaces_filter_action(cache, NAMESPACE_ID, namespace).await;

    crate::shell::page(
        &["Storage", "Claims"],
        current_path,
        div()
            .class("storage-claims main-page")
            .child(actions(
                "PersistentVolumeClaims",
                vec![namespace_select, prompt_action(FILTER_ID, filter)],
            ))
            .child(fragment(cache, namespace, filter).await),
    )
}

pub async fn fragment(cache: &CacheStore, namespace: &str, filter: &str) -> Element {
    let columns = columns();
    let rows = rows(cache, &columns, namespace, filter).await;

    data_list_view(&columns, &rows)
        .attr("id", FRAGMENT_ID)
        .attr(
            "hx-get",
            format!("{}/storage/claims/fragment", crate::base_path::ui_base()),
        )
        .attr("hx-trigger", POLL_TRIGGER)
        .attr("hx-include", INCLUDE)
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 2),
        TableColumn::new("Namespace", TableColumnType::Link, 3),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Age", TableColumnType::String, 3),
        TableColumn::new("Status", TableColumnType::String, 3),
        TableColumn::new("Class Name", TableColumnType::String, 3),
        TableColumn::new("Volume", TableColumnType::String, 3),
        TableColumn::new("Capacity", TableColumnType::String, 3),
    ]
}

async fn rows(
    cache: &CacheStore,
    columns: &[TableColumn],
    namespace: &str,
    filter: &str,
) -> Vec<TableRow> {
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/cluster/namespaces/".to_string();
    params[2] = "/storage/:1/claims/".to_string();

    let namespace_filter = if namespace.is_empty() || namespace == "All Namespaces" {
        None
    } else {
        Some(namespace.to_string())
    };

    let mut list = claims_api::get_claims(cache, namespace_filter)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|n| {
            n.metadata
                .name
                .to_lowercase()
                .contains(&filter.to_lowercase())
        })
        .map(|n| {
            vec![
                "PersistentVolume".to_string(),
                n.metadata.namespace.clone(),
                n.metadata.name.clone(),
                time_until_now(n.metadata.creation_timestamp.as_deref().unwrap_or_default()),
                n.status.phase,
                n.spec.storage_class_name,
                n.spec.volume_name,
                n.spec.resources.requests.storage,
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    parse_table_rows(columns, list, &styles, &params)
}
