use api::cluster::events as events_api;
use domain::utils::time::time_until_now;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::shared::data::data_list_view;
use crate::components::shared::table::{parse_table_rows, TableColumn, TableColumnType};

/// Events K8s recorded against a specific object (e.g. a Service, Deployment,
/// Pod), matched by `involved_object.kind`/`.name` - reused across every
/// detail page that shows an events tab.
pub async fn render(
    cache: &CacheStore,
    object_type: &str,
    namespace: &str,
    object_name: &str,
    fragment_url: &str,
) -> Element {
    let columns = columns();
    let namespace_filter = if namespace.is_empty() || namespace == "All Namespaces" {
        None
    } else {
        Some(namespace.to_string())
    };

    let mut list = events_api::get_events(cache, namespace_filter)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|e| e.involved_object.kind == object_type && e.involved_object.name == object_name)
        .map(|event| {
            vec![
                time_until_now(event.first_timestamp.as_deref().unwrap_or_default()),
                event.reason,
                event.message,
            ]
        })
        .collect::<Vec<_>>();
    list.sort_by(|a, b| a[0].cmp(&b[0]));

    let styles = vec![String::new(); columns.len()];
    let params = vec![String::new(); columns.len()];
    let rows = parse_table_rows(&columns, list, &styles, &params);

    data_list_view(&columns, &rows)
        .attr("id", "object-events")
        .attr("hx-get", fragment_url)
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Time", TableColumnType::String, 1),
        TableColumn::new("Reason", TableColumnType::String, 1),
        TableColumn::new("Event", TableColumnType::String, 3),
    ]
}
