use api::cluster::nodes as nodes_api;
use domain::utils::time::time_until_now;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

pub async fn fragment(cache: &CacheStore, node_name: &str) -> Element {
    let columns = columns();
    let rows = rows(cache, &columns, node_name).await;

    data_list_view(&columns, &rows)
        .attr("id", "node-conditions")
        .attr(
            "hx-get",
            format!(
                "{}/cluster/nodes/{node_name}/conditions/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Condition", TableColumnType::String, 1),
        TableColumn::new("Status", TableColumnType::String, 1),
        TableColumn::new("Transition", TableColumnType::String, 1),
        TableColumn::new("Reason", TableColumnType::String, 2),
        TableColumn::new("Message", TableColumnType::String, 2),
    ]
}

async fn rows(cache: &CacheStore, columns: &[TableColumn], node_name: &str) -> Vec<TableRow> {
    let styles = vec![String::new(); columns.len()];
    let params = vec![String::new(); columns.len()];

    let mut list = nodes_api::get_node_by_name(cache, node_name.to_string())
        .await
        .unwrap_or_default()
        .status
        .conditions
        .into_iter()
        .map(|r| {
            vec![
                r.r#type,
                r.status,
                time_until_now(&r.last_transition_time),
                r.reason,
                r.message,
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    parse_table_rows(columns, list, &styles, &params)
}
