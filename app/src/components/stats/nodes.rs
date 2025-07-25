use api::cluster::nodes as nodes_api;
use api::metrics as metrics_api;
use domain::cluster::node::Node;
use domain::metrics::NodeMetrics;
use domain::utils::time::time_until_now;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::stats::convert_memory;

#[component]
pub fn NodesStatComponent(
    #[prop(default = RwSignal::new(String::new()))] node_name: RwSignal<String>,
    #[prop(default = true)] expandable: bool,
) -> impl IntoView {
    let nodes_age = RwSignal::new(String::new());
    let nodes_ready = RwSignal::new((0., 0.));
    let nodes_cpu = RwSignal::new((0., 0.));
    let nodes_memory_values = RwSignal::new((0., 0.));
    let nodes_memory_labels = RwSignal::new((String::new(), String::new()));
    let expandable = RwSignal::new(expandable);

    let interval_handle = update_page_effect(10_000, move || {
        update_page(
            node_name,
            nodes_age,
            nodes_ready,
            nodes_cpu,
            nodes_memory_values,
            nodes_memory_labels,
        );
    });
    clear_page_effect(interval_handle);

    view(
        node_name,
        nodes_age,
        nodes_ready,
        nodes_cpu,
        nodes_memory_values,
        nodes_memory_labels,
        expandable,
    )
}

fn update_page(
    node_name: RwSignal<String>,
    nodes_age: RwSignal<String>,
    nodes_ready: RwSignal<(f64, f64)>,
    nodes_cpu: RwSignal<(f64, f64)>,
    nodes_memory_values: RwSignal<(f64, f64)>,
    nodes_memory_labels: RwSignal<(String, String)>,
) {
    if node_name.is_disposed() {
        return;
    }
    let node_name = node_name.get();

    spawn_local(async move {
        let node_name = if node_name.is_empty() {
            None
        } else {
            Some(node_name)
        };
        let nodes = nodes_api::get_nodes_filtered(node_name).await;
        let nodes_metrics = metrics_api::get_nodes()
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|n| nodes.iter().any(|s| s.metadata.name == n.metadata.name))
            .collect::<Vec<NodeMetrics>>();

        nodes_age.set(time_until_now(
            &nodes
                .iter()
                .map(|n| n.clone().metadata.creation_timestamp.unwrap_or_default())
                .min()
                .unwrap_or_default(),
        ));
        nodes_ready.set(get_nodes_ready(&nodes));
        nodes_cpu.set(get_nodes_cpu(&nodes, &nodes_metrics));
        let nodes_memory = get_nodes_memory(&nodes, &nodes_metrics);
        nodes_memory_values.set(nodes_memory.0);
        nodes_memory_labels.set(nodes_memory.1);
    });
}

fn view(
    node_name: RwSignal<String>,
    nodes_age: RwSignal<String>,
    nodes_ready: RwSignal<(f64, f64)>,
    nodes_cpu: RwSignal<(f64, f64)>,
    nodes_memory_values: RwSignal<(f64, f64)>,
    nodes_memory_labels: RwSignal<(String, String)>,
    expandable: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Show
            when=move || expandable.get()
            fallback=move || view_internal(
                node_name,
                nodes_age,
                nodes_ready,
                nodes_cpu,
                nodes_memory_values,
                nodes_memory_labels
            )>
            <Expandable label="Stats" expanded=true>
                <ExpandableSlot slot>
                    {view_internal(
                        node_name,
                        nodes_age,
                        nodes_ready,
                        nodes_cpu,
                        nodes_memory_values,
                        nodes_memory_labels
                    )}
                </ExpandableSlot>
            </Expandable>
        </Show>
    }
}

fn view_internal(
    node_name: RwSignal<String>,
    nodes_age: RwSignal<String>,
    nodes_ready: RwSignal<(f64, f64)>,
    nodes_cpu: RwSignal<(f64, f64)>,
    nodes_memory_values: RwSignal<(f64, f64)>,
    nodes_memory_labels: RwSignal<(String, String)>,
) -> impl IntoView {
    view! {
        <div class="card-container dcc-3">
            <Show
                when=move || node_name.get().is_empty()
                fallback=move || view! {
                    <CardString
                        label="Uptime"
                        label_add=""
                        value=nodes_age.get() />
                    }>
                <CardCircle
                    label="Nodes"
                    label_add="ready vs all"
                    values=nodes_ready.get() />

            </Show>
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
    }
}

fn get_nodes_ready(nodes: &[Node]) -> (f64, f64) {
    let ncount = nodes.len();
    let nready = nodes
        .iter()
        .filter(|s| {
            s.status
                .conditions
                .iter()
                .any(|c| c.r#type == "Ready" && c.status == "True")
        })
        .count();
    (ncount as f64, nready as f64)
}

fn get_nodes_cpu(nodes: &[Node], metrics: &[NodeMetrics]) -> (f64, f64) {
    let ncap = nodes.iter().fold(0., |acc, node| {
        acc + node.status.capacity.cpu.parse::<f64>().unwrap_or(0.)
    });
    let nuse = metrics.iter().fold(0., |acc, node| {
        acc + node
            .usage
            .cpu
            .trim_end_matches('n')
            .parse::<f64>()
            .unwrap_or(0.)
    });
    (ncap, nuse / 1_000_000_000.)
}

fn get_nodes_memory(nodes: &[Node], metrics: &[NodeMetrics]) -> ((f64, f64), (String, String)) {
    let ncap = convert_memory(nodes.iter().fold(0., |acc, node| {
        acc + crate::utils::stats::parse_memory(&node.status.capacity.memory).unwrap_or(0.)
    }));
    let nuse = convert_memory(metrics.iter().fold(0., |acc, node| {
        acc + crate::utils::stats::parse_memory(&node.usage.memory).unwrap_or(0.)
    }));
    ((ncap.0, nuse.0), (ncap.1, nuse.1))
}
