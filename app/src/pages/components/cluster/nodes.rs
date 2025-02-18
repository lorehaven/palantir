use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::domain::metrics::NodeMetrics;
use crate::domain::node::{Node, NodeType};
use crate::domain::pod::Pod;
use crate::pages::components::prelude::*;
use crate::pages::utils::shared::effects::update_page_effect;
use crate::pages::utils::shared::time::time_until_now;
use crate::pages::utils::stats::{convert_memory, parse_memory};

#[component]
pub fn NodesListComponent() -> impl IntoView {
    let nodes = RwSignal::new(vec![]);

    update_page_effect(5_000, move || update_page(nodes));
    view(nodes)
}

fn update_page(
    nodes: RwSignal<Vec<Vec<String>>>,
) {
    spawn_local(async move {
        let nodes_data = crate::api::nodes::get_nodes().await.unwrap_or_default();
        let nodes_metrics = crate::api::metrics::get_nodes().await.unwrap_or_default();
        let pods_data = crate::api::pods::get_pods().await.unwrap_or_default();

        let mut nodes_vec = vec![];
        for node in nodes_data {
            let mut vec = vec![];
            let node_metric = nodes_metrics.clone().into_iter().find(|nm| nm.get_node_name() == node.metadata.name).unwrap();
            let node_pods_data = pods_data.clone().into_iter().filter(|p| p.spec.node_name == node.metadata.name).collect::<Vec<Pod>>();
            vec.push(NodeType::from_node(&node).to_string());
            vec.push(node.clone().metadata.name);
            vec.push(time_until_now(&node.metadata.creation_timestamp));
            vec.push(node.clone().metadata.labels.into_iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join("\n"));
            vec.push(node.clone().status.conditions.iter().any(|c| c.r#type == "Ready" && c.status == "True").to_string());
            vec.push(get_node_cpu_actual(&node, &node_metric));
            let node_cpu_requests_limits = get_node_cpu_requests_limits(&node, &node_pods_data);
            vec.push(node_cpu_requests_limits.0);
            vec.push(node_cpu_requests_limits.1);
            vec.push(get_node_memory_actual(&node, &node_metric));
            let node_memory_requests_limits = get_node_memory_requests_limits(&node, &node_pods_data);
            vec.push(node_memory_requests_limits.0);
            vec.push(node_memory_requests_limits.1);
            nodes_vec.push(vec);
        }
        nodes.set(nodes_vec);
    });
}

fn view(
    nodes: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    view! {
        <Expandable label="" expanded=true>
            <ExpandableSlot slot>
                <div class="card-container dcc-1">
                    <CardList
                        labels=&["Type", "Name", "Age", "Labels", "Ready", "CPU actual", "CPU requested", "CPU limits", "RAM actual", "RAM requested", "RAM limits"]
                        widths=&["5%", "11%", "5%", "20%", "5%", "9%", "9%", "9%", "9%", "9%", "9%"]
                        rows=nodes.get() />
                </div>
            </ExpandableSlot>
        </Expandable>
    }
}

fn get_node_cpu_actual(node: &Node, metrics: &NodeMetrics) -> String {
    let ncap = node.status.capacity.cpu.parse::<f64>().unwrap_or(0.);
    let nuse = metrics.usage.cpu.trim_end_matches('n').parse::<f64>().unwrap_or(0.) / 1_000_000_000.;
    format!("{:.2}%\n{:.2}", nuse / ncap * 100., nuse * 1000.)
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
    if request.ends_with("m") {
        request.trim_end_matches("m").parse().unwrap_or(0.)
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
