use api::cluster::events as events_api;
use domain::utils::time::time_until_now;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::nodes as nodes_stat;
use crate::components::stats::pods as pods_stat;

pub async fn render(cache: &CacheStore, current_path: &str) -> String {
    crate::shell::page(
        &[],
        current_path,
        div()
            .class("dashboard main-page")
            .child(nodes_stat::render(cache, None).await)
            .child(pods_stat::render(cache, None, None).await)
            .child(events_fragment(cache).await),
    )
}

pub async fn events_fragment(cache: &CacheStore) -> Element {
    let columns = columns();
    let rows = rows(cache, &columns).await;

    data_list_view(&columns, &rows)
        .attr("id", "dashboard-events")
        .attr(
            "hx-get",
            format!("{}/events/fragment", crate::base_path::ui_base()),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        // Not a Link: events cover every resource kind cluster-wide, and
        // each kind needs a differently-shaped URL - a single per-column
        // link template (see `table::parse_table_rows`) can't route all of
        // them, so a Link here rendered `{ui_base}{name}` with no path in
        // between and 404'd.
        TableColumn::new("Name", TableColumnType::String, 2),
        TableColumn::new("Time", TableColumnType::String, 1),
        TableColumn::new("Reason", TableColumnType::String, 1),
        TableColumn::new("Event", TableColumnType::String, 3),
    ]
}

async fn rows(cache: &CacheStore, columns: &[TableColumn]) -> Vec<TableRow> {
    let styles = vec![String::new(); columns.len()];
    let params = vec![String::new(); columns.len()];

    let mut list = events_api::get_events(cache, None)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| {
            vec![
                r.involved_object.kind,
                r.involved_object.name,
                time_until_now(r.first_timestamp.as_deref().unwrap_or_default()),
                r.reason,
                r.message,
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[2].cmp(&b[2]));
    parse_table_rows(columns, list, &styles, &params)
}
