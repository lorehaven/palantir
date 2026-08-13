use api::storage::volumes as volumes_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const FRAGMENT_ID: &str = "volumes-table";
const FILTER_ID: &str = "volumes-filter";
const POLL_TRIGGER: &str = "every 10s, keyup changed delay:300ms from:#volumes-filter";

pub async fn render(cache: &CacheStore, current_path: &str, filter: &str) -> String {
    crate::shell::page(
        &["Storage", "Volumes"],
        current_path,
        div()
            .class("storage-volumes main-page")
            .child(actions(
                "PersistentVolumes",
                vec![prompt_action(FILTER_ID, filter)],
            ))
            .child(fragment(cache, filter).await),
    )
}

pub async fn fragment(cache: &CacheStore, filter: &str) -> Element {
    let columns = columns();
    let rows = rows(cache, &columns, filter).await;

    data_list_view(&columns, &rows)
        .attr("id", FRAGMENT_ID)
        .attr(
            "hx-get",
            format!("{}/storage/volumes/fragment", crate::base_path::ui_base()),
        )
        .attr("hx-trigger", POLL_TRIGGER)
        .attr("hx-include", format!("#{FILTER_ID}"))
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 2),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Status", TableColumnType::String, 3),
        TableColumn::new("Capacity", TableColumnType::String, 3),
    ]
}

async fn rows(cache: &CacheStore, columns: &[TableColumn], filter: &str) -> Vec<TableRow> {
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/storage/volumes/".to_string();

    let mut list = volumes_api::get_volumes(cache)
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
                n.metadata.name.clone(),
                n.status.phase,
                n.spec.capacity.storage,
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    parse_table_rows(columns, list, &styles, &params)
}
