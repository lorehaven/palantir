use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::domain::metrics::NodeMetrics;
use crate::domain::node::Node;
use crate::pages::components::prelude::*;
use crate::pages::utils::shared::effects::update_page_effect;
use crate::pages::utils::stats::convert_memory;

#[component]
pub fn NodesStatComponent() -> impl IntoView {
    let nodes_ready = RwSignal::new((0., 0.));
    let nodes_cpu = RwSignal::new((0., 0.));
    let nodes_memory_values = RwSignal::new((0., 0.));
    let nodes_memory_labels = RwSignal::new((String::new(), String::new()));

    update_page_effect(5_000, move || update_page(
        nodes_ready,
        nodes_cpu,
        nodes_memory_values,
        nodes_memory_labels,));
    view(
        nodes_ready,
        nodes_cpu,
        nodes_memory_values,
        nodes_memory_labels,)
}

fn update_page(
    nodes_ready: RwSignal<(f64, f64)>,
    nodes_cpu: RwSignal<(f64, f64)>,
    nodes_memory_values: RwSignal<(f64, f64)>,
    nodes_memory_labels: RwSignal<(String, String)>,
) {
    spawn_local(async move {
        let nodes = crate::api::nodes::get_nodes().await.unwrap_or_default();
        let nodes_metrics = crate::api::metrics::get_nodes().await.unwrap_or_default();
        nodes_ready.set(get_nodes_ready(&nodes));
        nodes_cpu.set(get_nodes_cpu(&nodes, &nodes_metrics));
        let nodes_memory = get_nodes_memory(&nodes, &nodes_metrics);
        nodes_memory_values.set(nodes_memory.0);
        nodes_memory_labels.set(nodes_memory.1);
    });
}

fn view(
    nodes_ready: RwSignal<(f64, f64)>,
    nodes_cpu: RwSignal<(f64, f64)>,
    nodes_memory_values: RwSignal<(f64, f64)>,
    nodes_memory_labels: RwSignal<(String, String)>,
) -> impl IntoView {
    view! {
        <Expandable label="Nodes" expanded=true>
            <ExpandableSlot slot>
                <div class="card-container dcc-3">
                    <CardCircle
                        label="Nodes"
                        label_add="ready vs all"
                        values=nodes_ready.get() />
                    <CardCircle
                        label="Node CPU usage"
                        label_add="used vs available"
                        values=nodes_cpu.get()
                        decimal=false />
                    <CardCircle
                        label="Node Memory usage"
                        label_add="used vs available"
                        values=nodes_memory_values.get()
                        value_labels=nodes_memory_labels.get()
                        decimal=false />
                </div>
            </ExpandableSlot>
        </Expandable>
    }
}

fn get_nodes_ready(nodes: &[Node]) -> (f64, f64) {
    let ncount = nodes.len();
    let nready = nodes.iter()
        .filter(|s| s.status.conditions.iter().any(|c| c.r#type == "Ready" && c.status == "True"))
        .count();
    (ncount as f64, nready as f64)
}

fn get_nodes_cpu(nodes: &[Node], metrics: &[NodeMetrics]) -> (f64, f64) {
    let ncap = nodes.iter().fold(0., |acc, node| acc + node.status.capacity.cpu.parse::<f64>().unwrap_or(0.));
    let nuse = metrics.iter().fold(0., |acc, node| acc + node.usage.cpu.trim_end_matches('n').parse::<f64>().unwrap_or(0.));
    (ncap, nuse / 1_000_000_000.)
}

fn get_nodes_memory(nodes: &[Node], metrics: &[NodeMetrics]) -> ((f64, f64), (String, String)) {
    let ncap = convert_memory(nodes.iter().fold(0., |acc, node| acc + crate::pages::utils::stats::parse_memory(&node.status.capacity.memory).unwrap_or(0.)));
    let nuse = convert_memory(metrics.iter().fold(0., |acc, node| acc + crate::pages::utils::stats::parse_memory(&node.usage.memory).unwrap_or(0.)));
    ((ncap.0, nuse.0), (ncap.1, nuse.1))
}
