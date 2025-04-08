use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::metrics as metrics_api;
use crate::api::cluster::nodes as nodes_api;
use crate::api::workloads::pods as pods_api;
use crate::domain::metrics::NodeMetrics;
use crate::domain::cluster::node::{Node, NodeType};
use crate::domain::cluster::pod::Pod;
use crate::components::prelude::*;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::time_until_now;
use crate::pages::utils::stats::{convert_memory, parse_memory};

#[component]
pub fn NodesListComponent(
    prompt: RwSignal<String>,
) -> impl IntoView {
    let nodes = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(nodes, prompt));
    clear_page_effect(interval_handle);
    view(nodes)
}

fn update_page(
    nodes: RwSignal<Vec<Vec<String>>>,
    prompt: RwSignal<String>,
) {
    if prompt.is_disposed() { return; }
    let prompt_value = prompt.get();

    spawn_local(async move {
        let nodes_data = nodes_api::get_nodes().await
            .unwrap_or_default()
            .into_iter()
            .filter(|n| n.metadata.name.to_lowercase().contains(&prompt_value.to_lowercase()))
            .collect::<Vec<_>>();
        let nodes_metrics = metrics_api::get_nodes().await.unwrap_or_default()
            .into_iter()
            .filter(|n| nodes_data.iter().any(|s| s.metadata.name == n.metadata.name))
            .collect::<Vec<NodeMetrics>>();
        let pods_data = pods_api::get_pods().await.unwrap_or_default();

        let mut nodes_vec = vec![];
        for node in nodes_data {
            let node_metric = nodes_metrics.clone().into_iter().find(|nm| nm.get_node_name() == node.metadata.name).unwrap();
            let node_pods_data = pods_data.clone().into_iter().filter(|p| p.spec.node_name == node.metadata.name).collect::<Vec<Pod>>();
            let node_cpu_requests_limits = get_node_cpu_requests_limits(&node, &node_pods_data);
            let node_memory_requests_limits = get_node_memory_requests_limits(&node, &node_pods_data);
            nodes_vec.push(vec![
                NodeType::from_node(&node).to_string(),
                node.clone().metadata.name,
                time_until_now(&node.clone().metadata.creation_timestamp.unwrap_or_default()),
                node.clone().metadata.labels.into_iter().map(|(k, v)| format!("{k}: {v}")).collect::<Vec<String>>().join("\n"),
                node.clone().status.conditions.iter().any(|c| c.r#type == "Ready" && c.status == "True").to_string(),
                get_node_cpu_actual(&node, &node_metric),
                node_cpu_requests_limits.0,
                node_cpu_requests_limits.1,
                get_node_memory_actual(&node, &node_metric),
                node_memory_requests_limits.0,
                node_memory_requests_limits.1,
            ]);
        }
        nodes.set(nodes_vec);
    });
}

fn view(
    nodes: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
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
    ];
    let mut styles = vec![""; columns.len()];
    styles[4] = "font-size: 1.6rem;";
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/nodes/";

    view! {
        <Expandable label="" expanded=true>
            <ExpandableSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=nodes.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </ExpandableSlot>
        </Expandable>
    }
}

fn get_node_cpu_actual(node: &Node, metrics: &NodeMetrics) -> String {
    let ncap = node.status.capacity.cpu.parse::<f64>().unwrap_or(0.);
    let nuse = metrics.usage.cpu.trim_end_matches('n').parse::<f64>().unwrap_or(0.) / 1_000_000_000.;
    format!("{:.2}%\n{:.2}m", nuse / ncap * 100., nuse * 1000.)
}

fn get_node_cpu_requests_limits(node: &Node, pods: &[Pod]) -> (String, String) {
    let ncap = node.status.capacity.cpu.parse::<f64>().unwrap_or(0.);
    let requests = pods.iter()
        .fold(0., |acc, p| acc + p.spec.containers.iter()
            .fold(0., |acc, c| acc + parse_pod_cpu(&c.resources.requests.cpu)));
    let limits = pods.iter()
        .fold(0., |acc, p| acc + p.spec.containers.iter()
            .fold(0., |acc, c| acc + parse_pod_cpu(&c.resources.limits.cpu)));
    (format!("{:.2}%\n{:.0}m", requests / ncap / 10., requests), format!("{:.2}%\n{:.0}m", limits / ncap / 10., limits))
}

fn parse_pod_cpu(request: &str) -> f64 {
    if request.ends_with('m') {
        request.trim_end_matches('m').parse().unwrap_or(0.)
    } else {
        request.parse::<f64>().unwrap_or(0.) * 1000.
    }
}

fn get_node_memory_actual(node: &Node, metrics: &NodeMetrics) -> String {
    let ncap = convert_memory(parse_memory(&node.status.capacity.memory).unwrap_or(0.));
    let nuse = convert_memory(parse_memory(&metrics.usage.memory).unwrap_or(0.));
    format!("{:.2}%\n{:.2}{}", nuse.0 / ncap.0 * 100., nuse.0, nuse.1)
}

fn get_node_memory_requests_limits(node: &Node, pods: &[Pod]) -> (String, String) {
    let ncap = convert_memory(parse_memory(&node.status.capacity.memory).unwrap_or(0.));
    let requests = pods.iter()
        .fold(0., |acc, p| acc + p.spec.containers.iter()
            .filter(|c| !c.resources.requests.memory.is_empty())
            .fold(0., |acc, c| acc + parse_memory(&c.resources.requests.memory).unwrap_or_default()));
    let requests = convert_memory(requests);
    let limits = pods.iter()
        .fold(0., |acc, p| acc + p.spec.containers.iter()
            .filter(|c| !c.resources.limits.memory.is_empty())
            .fold(0., |acc, c| acc + parse_memory(&c.resources.limits.memory).unwrap_or_default()));
    let limits = convert_memory(limits);
    (format!("{:.2}%\n{:.2}{}", requests.0 / ncap.0 * 100., requests.0, requests.1), format!("{:.2}%\n{:.2}{}", limits.0 / ncap.0 * 100., limits.0, limits.1))
}
