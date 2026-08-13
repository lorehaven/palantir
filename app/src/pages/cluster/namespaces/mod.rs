use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const FRAGMENT_ID: &str = "namespaces-table";
const FILTER_ID: &str = "ns-filter";
const POLL_TRIGGER: &str = "every 10s, keyup changed delay:300ms from:#ns-filter";

pub async fn render(cache: &CacheStore, current_path: &str, filter: &str) -> String {
    crate::shell::page(
        &["Cluster", "Namespaces"],
        current_path,
        div()
            .class("cluster-namespaces main-page")
            .child(actions(
                "Namespaces",
                vec![prompt_action(FILTER_ID, filter)],
            ))
            .child(fragment(cache, filter).await),
    )
}

/// Rendered both inline (first paint, inside `render`) and standalone.
///
/// The `/fragment` route htmx polls/filters against reuses this too - see
/// the module doc on `actions::prompt::prompt_action` for why the filter
/// input itself carries no htmx attributes.
pub async fn fragment(cache: &CacheStore, filter: &str) -> Element {
    let columns = columns();
    let rows = rows(cache, &columns, filter).await;

    data_list_view(&columns, &rows)
        .attr("id", FRAGMENT_ID)
        .attr(
            "hx-get",
            format!(
                "{}/cluster/namespaces/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", POLL_TRIGGER)
        .attr("hx-include", format!("#{FILTER_ID}"))
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::Link, 2),
        TableColumn::new("Age", TableColumnType::String, 1),
        TableColumn::new("Status", TableColumnType::String, 1),
    ]
}

async fn rows(cache: &CacheStore, columns: &[TableColumn], filter: &str) -> Vec<TableRow> {
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/cluster/namespaces/".to_string();

    let mut list = api::cluster::namespaces::get_namespaces(cache)
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
                "Namespace".to_string(),
                n.metadata.name.clone(),
                domain::utils::time::time_until_now(
                    n.metadata.creation_timestamp.as_deref().unwrap_or_default(),
                ),
                n.status.phase,
            ]
        })
        .collect::<Vec<_>>();
    list.sort_by(|a, b| a[1].cmp(&b[1]));

    parse_table_rows(columns, list, &styles, &params)
}
