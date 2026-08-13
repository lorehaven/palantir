use api::workloads::ingresses as ingresses_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const FRAGMENT_ID: &str = "ingresses-table";
const FILTER_ID: &str = "ingresses-filter";
const NAMESPACE_ID: &str = "ingresses-namespace";
const POLL_TRIGGER: &str =
    "every 10s, keyup changed delay:300ms from:#ingresses-filter, change from:#ingresses-namespace";
const INCLUDE: &str = "#ingresses-filter, #ingresses-namespace";

pub async fn render(
    cache: &CacheStore,
    current_path: &str,
    namespace: &str,
    filter: &str,
) -> String {
    let namespace_select = namespaces_filter_action(cache, NAMESPACE_ID, namespace).await;

    crate::shell::page(
        &["Workloads", "Ingresses"],
        current_path,
        div()
            .class("workloads-ingresses main-page")
            .child(actions(
                "Ingresses",
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
                "{}/workloads/ingresses/fragment",
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
        TableColumn::new("Namespace", TableColumnType::Link, 3),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Hosts", TableColumnType::StringList, 3),
        TableColumn::new("Paths", TableColumnType::StringList, 3),
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
    params[2] = "/workloads/:1/ingresses/".to_string();

    let namespace_filter = if namespace.is_empty() || namespace == "All Namespaces" {
        None
    } else {
        Some(namespace.to_string())
    };

    let mut list = ingresses_api::get_ingresses(cache, namespace_filter)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|i| i.metadata.name.contains(filter))
        .map(|ingress| {
            let hosts = ingress
                .spec
                .rules
                .iter()
                .map(|r| r.host.clone())
                .collect::<Vec<_>>()
                .join("\n");
            let paths = ingress
                .spec
                .rules
                .iter()
                .map(|r| {
                    r.http
                        .paths
                        .iter()
                        .map(|p| p.path.clone())
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .collect::<Vec<_>>()
                .join("\n");

            vec![
                "Ingress".to_string(),
                ingress.metadata.namespace.clone(),
                ingress.metadata.name,
                hosts,
                paths,
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    parse_table_rows(columns, list, &styles, &params)
}
