use api::workloads::services as services_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const FRAGMENT_ID: &str = "services-table";
const FILTER_ID: &str = "services-filter";
const NAMESPACE_ID: &str = "services-namespace";
const POLL_TRIGGER: &str =
    "every 10s, keyup changed delay:300ms from:#services-filter, change from:#services-namespace";
const INCLUDE: &str = "#services-filter, #services-namespace";

pub async fn render(
    cache: &CacheStore,
    current_path: &str,
    namespace: &str,
    filter: &str,
) -> String {
    let namespace_select = namespaces_filter_action(cache, NAMESPACE_ID, namespace).await;

    crate::shell::page(
        &["Workloads", "Services"],
        current_path,
        div()
            .class("workloads-services main-page")
            .child(actions(
                "Services",
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
            format!(
                "{}/workloads/services/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", POLL_TRIGGER)
        .attr("hx-include", INCLUDE)
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 2),
        TableColumn::new("Name", TableColumnType::Link, 7),
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
    params[2] = "/workloads/:1/services/".to_string();

    let namespace_filter = if namespace.is_empty() || namespace == "All Namespaces" {
        None
    } else {
        Some(namespace.to_string())
    };

    let mut list = services_api::get_services(cache, namespace_filter)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|s| {
            s.metadata
                .name
                .to_lowercase()
                .contains(&filter.to_lowercase())
        })
        .map(|n| {
            vec![
                "Service".to_string(),
                n.metadata.namespace.clone(),
                n.metadata.name,
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    parse_table_rows(columns, list, &styles, &params)
}
