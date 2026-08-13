use api::cluster::nodes as nodes_api;
use api::metrics as metrics_api;
use api::workloads::pods as pods_api;
use domain::cluster::node::{Node, NodeType};
use domain::cluster::pod::Pod;
use domain::metrics::NodeMetrics;
use domain::utils::time::time_until_now;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::nodes as nodes_stat;
use crate::utils::stats::{convert_memory, parse_memory};

const FRAGMENT_ID: &str = "nodes-table";
const FILTER_ID: &str = "nodes-filter";
const POLL_TRIGGER: &str = "every 10s, keyup changed delay:300ms from:#nodes-filter";

pub async fn render(cache: &CacheStore, current_path: &str, filter: &str) -> String {
    crate::shell::page(
        &["Cluster", "Nodes"],
        current_path,
        div()
            .class("cluster-nodes main-page")
            .child(actions("Nodes", vec![prompt_action(FILTER_ID, filter)]))
            .child(nodes_stat::render(cache, None).await)
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
            format!("{}/cluster/nodes/fragment", crate::base_path::ui_base()),
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
        TableColumn::new("Labels", TableColumnType::StringList, 4),
        TableColumn::new("Ready", TableColumnType::Bool, 1),
        TableColumn::new("CPU actual", TableColumnType::StringTwoLine, 2),
        TableColumn::new("CPU requested", TableColumnType::StringTwoLine, 2),
        TableColumn::new("CPU limits", TableColumnType::StringTwoLine, 2),
        TableColumn::new("RAM actual", TableColumnType::StringTwoLine, 2),
        TableColumn::new("RAM requested", TableColumnType::StringTwoLine, 2),
        TableColumn::new("RAM limits", TableColumnType::StringTwoLine, 2),
    ]
}

async fn rows(cache: &CacheStore, columns: &[TableColumn], filter: &str) -> Vec<TableRow> {
    let mut styles = vec![String::new(); columns.len()];
    styles[4] = "font-size: 1.6rem;".to_string();
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/cluster/nodes/".to_string();

    let nodes_data = nodes_api::get_nodes(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|n| {
            n.metadata
                .name
                .to_lowercase()
                .contains(&filter.to_lowercase())
        })
        .collect::<Vec<_>>();
    let nodes_metrics = metrics_api::get_nodes(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|n| {
            nodes_data
                .iter()
                .any(|s| s.metadata.name == n.metadata.name)
        })
        .collect::<Vec<NodeMetrics>>();
    let pods_data = pods_api::get_pods(cache, None, None)
        .await
        .unwrap_or_default();

    let mut list = nodes_data
        .into_iter()
        .map(|node| node_row(&node, &nodes_metrics, &pods_data))
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    parse_table_rows(columns, list, &styles, &params)
}

fn node_row(node: &Node, nodes_metrics: &[NodeMetrics], pods_data: &[Pod]) -> Vec<String> {
    let node_metric = nodes_metrics
        .iter()
        .find(|nm| nm.get_node_name() == node.metadata.name)
        .cloned()
        .unwrap_or_default();
    let node_pods_data = pods_data
        .iter()
        .filter(|p| p.spec.node_name == node.metadata.name)
        .cloned()
        .collect::<Vec<Pod>>();
    let cpu = node_cpu_requests_limits(node, &node_pods_data);
    let memory = node_memory_requests_limits(node, &node_pods_data);

    vec![
        NodeType::from_node(node).to_string(),
        node.metadata.name.clone(),
        time_until_now(
            node.metadata
                .creation_timestamp
                .as_deref()
                .unwrap_or_default(),
        ),
        node.metadata
            .labels
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<String>>()
            .join("\n"),
        node.status
            .conditions
            .iter()
            .any(|c| c.r#type == "Ready" && c.status == "True")
            .to_string(),
        node_cpu_actual(node, &node_metric),
        cpu.0,
        cpu.1,
        node_memory_actual(node, &node_metric),
        memory.0,
        memory.1,
    ]
}

/// Deliberately distinct from `utils::stats::parse_pod_cpu` (which returns
/// whole cores) - this one returns millicores, matching the percentage math
/// below. Ported as-is from the original Leptos `nodes_list.rs`, which had
/// the same two-different-units split.
fn parse_pod_cpu_millicores(request: &str) -> f64 {
    if request.ends_with('m') {
        request.trim_end_matches('m').parse().unwrap_or(0.)
    } else {
        request.parse::<f64>().unwrap_or(0.) * 1000.
    }
}

fn node_cpu_actual(node: &Node, metrics: &NodeMetrics) -> String {
    let ncap = node.status.capacity.cpu.parse::<f64>().unwrap_or(0.);
    let nuse = metrics
        .usage
        .cpu
        .trim_end_matches('n')
        .parse::<f64>()
        .unwrap_or(0.)
        / 1_000_000_000.;
    format!("{:.2}%\n{:.2}m", nuse / ncap * 100., nuse * 1000.)
}

fn node_cpu_requests_limits(node: &Node, pods: &[Pod]) -> (String, String) {
    let ncap = node.status.capacity.cpu.parse::<f64>().unwrap_or(0.);
    let requests = pods.iter().fold(0., |acc, p| {
        acc + p.spec.containers.iter().fold(0., |acc, c| {
            acc + parse_pod_cpu_millicores(&c.resources.requests.cpu)
        })
    });
    let limits = pods.iter().fold(0., |acc, p| {
        acc + p.spec.containers.iter().fold(0., |acc, c| {
            acc + parse_pod_cpu_millicores(&c.resources.limits.cpu)
        })
    });
    (
        format!("{:.2}%\n{:.0}m", requests / ncap / 10., requests),
        format!("{:.2}%\n{:.0}m", limits / ncap / 10., limits),
    )
}

fn node_memory_actual(node: &Node, metrics: &NodeMetrics) -> String {
    let ncap = convert_memory(parse_memory(&node.status.capacity.memory).unwrap_or(0.));
    let nuse = convert_memory(parse_memory(&metrics.usage.memory).unwrap_or(0.));
    format!("{:.2}%\n{:.2}{}", nuse.0 / ncap.0 * 100., nuse.0, nuse.1)
}

fn node_memory_requests_limits(node: &Node, pods: &[Pod]) -> (String, String) {
    let ncap = convert_memory(parse_memory(&node.status.capacity.memory).unwrap_or(0.));
    let requests = pods.iter().fold(0., |acc, p| {
        acc + p
            .spec
            .containers
            .iter()
            .filter(|c| !c.resources.requests.memory.is_empty())
            .fold(0., |acc, c| {
                acc + parse_memory(&c.resources.requests.memory).unwrap_or_default()
            })
    });
    let requests = convert_memory(requests);
    let limits = pods.iter().fold(0., |acc, p| {
        acc + p
            .spec
            .containers
            .iter()
            .filter(|c| !c.resources.limits.memory.is_empty())
            .fold(0., |acc, c| {
                acc + parse_memory(&c.resources.limits.memory).unwrap_or_default()
            })
    });
    let limits = convert_memory(limits);
    (
        format!(
            "{:.2}%\n{:.2}{}",
            requests.0 / ncap.0 * 100.,
            requests.0,
            requests.1
        ),
        format!(
            "{:.2}%\n{:.2}{}",
            limits.0 / ncap.0 * 100.,
            limits.0,
            limits.1
        ),
    )
}
