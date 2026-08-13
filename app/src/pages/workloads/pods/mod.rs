use api::metrics as metrics_api;
use api::workloads::pods as pods_api;
use domain::metrics::PodMetrics;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::pods as pods_stat;
use crate::utils::stats::pod_stats::{
    pod_cpu_actual, pod_cpu_limit, pod_cpu_request, pod_memory_actual, pod_memory_limit,
    pod_memory_request,
};

const FRAGMENT_ID: &str = "pods-table";
const FILTER_ID: &str = "pods-filter";
const NAMESPACE_ID: &str = "pods-namespace";
const POLL_TRIGGER: &str =
    "every 10s, keyup changed delay:300ms from:#pods-filter, change from:#pods-namespace";
const INCLUDE: &str = "#pods-filter, #pods-namespace";

pub async fn render(
    cache: &CacheStore,
    current_path: &str,
    namespace: &str,
    filter: &str,
) -> String {
    let namespace_select = namespaces_filter_action(cache, NAMESPACE_ID, namespace).await;
    let namespace_filter = namespace_filter(namespace);

    crate::shell::page(
        &["Workloads", "Pods"],
        current_path,
        div()
            .class("workloads-pods main-page")
            .child(actions(
                "Pods",
                vec![namespace_select, prompt_action(FILTER_ID, filter)],
            ))
            .child(pods_stat::render(cache, namespace_filter.as_deref(), None).await)
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
            format!("{}/workloads/pods/fragment", crate::base_path::ui_base()),
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
        TableColumn::new("CPU actual", TableColumnType::String, 1),
        TableColumn::new("CPU request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("CPU limit", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM actual", TableColumnType::String, 1),
        TableColumn::new("RAM request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM limit", TableColumnType::StringTwoLine, 1),
    ]
}

fn namespace_filter(namespace: &str) -> Option<String> {
    if namespace.is_empty() || namespace == "All Namespaces" {
        None
    } else {
        Some(namespace.to_string())
    }
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
    params[2] = "/workloads/:1/pods/".to_string();

    let pods_data = pods_api::get_pods(cache, namespace_filter(namespace), None)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|p| p.metadata.name.contains(filter))
        .collect::<Vec<_>>();
    let pod_names = pods_data
        .iter()
        .map(|p| p.metadata.name.clone())
        .collect::<Vec<String>>();
    let pods_metrics = metrics_api::get_pods(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|pm| pod_names.contains(&pm.metadata.name))
        .collect::<Vec<PodMetrics>>();

    let mut list = pods_data
        .into_iter()
        .map(|r| {
            let metrics = pods_metrics
                .iter()
                .find(|p| p.metadata.name == r.metadata.name)
                .cloned()
                .unwrap_or_default();

            vec![
                "Pod".to_string(),
                r.metadata.namespace.clone(),
                r.metadata.name.clone(),
                pod_cpu_actual(&metrics),
                pod_cpu_request(&r, &metrics),
                pod_cpu_limit(&r, &metrics),
                pod_memory_actual(&metrics),
                pod_memory_request(&r, &metrics),
                pod_memory_limit(&r, &metrics),
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    parse_table_rows(columns, list, &styles, &params)
}
