use api::metrics as metrics_api;
use api::workloads::pods as pods_api;
use domain::metrics::PodMetrics;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::stats::pod_stats::{
    pod_cpu_actual, pod_cpu_limit, pod_cpu_request, pod_memory_actual, pod_memory_limit,
    pod_memory_request,
};

pub async fn fragment(cache: &CacheStore, node_name: &str) -> Element {
    let columns = columns();
    let rows = rows(cache, &columns, node_name).await;

    data_list_view(&columns, &rows)
        .attr("id", "node-pods")
        .attr(
            "hx-get",
            format!(
                "{}/cluster/nodes/{node_name}/pods/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Namespace", TableColumnType::String, 2),
        TableColumn::new("Age", TableColumnType::String, 1),
        TableColumn::new("Restarts", TableColumnType::String, 1),
        TableColumn::new("CPU actual", TableColumnType::String, 1),
        TableColumn::new("CPU request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("CPU limit", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM actual", TableColumnType::String, 1),
        TableColumn::new("RAM request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM limit", TableColumnType::StringTwoLine, 1),
    ]
}

async fn rows(cache: &CacheStore, columns: &[TableColumn], node_name: &str) -> Vec<TableRow> {
    use domain::utils::time::time_until_now;

    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/workloads/:2/pods/".to_string();

    let pods_data = pods_api::get_pods(cache, None, Some(node_name.to_string()))
        .await
        .unwrap_or_default();
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
                .find(|pp| pp.metadata.name == r.metadata.name)
                .cloned()
                .unwrap_or_default();

            vec![
                "Pod".to_string(),
                r.metadata.name.clone(),
                r.metadata.namespace.clone(),
                time_until_now(r.metadata.creation_timestamp.as_deref().unwrap_or_default()),
                r.status
                    .container_statuses
                    .iter()
                    .map(|c| c.restart_count)
                    .sum::<i32>()
                    .to_string(),
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
