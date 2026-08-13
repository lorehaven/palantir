use api::cluster::nodes as nodes_api;
use api::metrics as metrics_api;
use domain::cluster::node::Node;
use domain::metrics::NodeMetrics;
use domain::utils::time::time_until_now;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::shared::card::card_circle::card_circle;
use crate::components::shared::card::card_string::card_string;
use crate::utils::stats::{convert_memory, parse_memory};

/// `node_name` scopes to a single node (showing its uptime instead of the
/// ready-count ring); `None` covers every node.
pub async fn render(cache: &CacheStore, node_name: Option<&str>) -> Element {
    let nodes = nodes_api::get_nodes_filtered(cache, node_name.map(ToString::to_string)).await;
    let nodes_metrics = metrics_api::get_nodes(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|n| nodes.iter().any(|s| s.metadata.name == n.metadata.name))
        .collect::<Vec<NodeMetrics>>();

    let nodes_cpu = get_nodes_cpu(&nodes, &nodes_metrics);
    let (nodes_memory_values, nodes_memory_labels) = get_nodes_memory(&nodes, &nodes_metrics);

    let first_card = if node_name.is_some() {
        let age = time_until_now(
            nodes
                .iter()
                .filter_map(|n| n.metadata.creation_timestamp.clone())
                .min()
                .unwrap_or_default()
                .as_str(),
        );
        card_string("Uptime", "", &age)
    } else {
        card_circle(
            "Nodes",
            "ready vs all",
            get_nodes_ready(&nodes),
            ("", ""),
            true,
        )
    };

    div()
        .class("card-container dcc-3")
        .child(first_card)
        .child(card_circle(
            "Node CPU usage",
            "used vs available",
            nodes_cpu,
            ("", ""),
            false,
        ))
        .child(card_circle(
            "Node Memory usage",
            "used vs available",
            nodes_memory_values,
            (&nodes_memory_labels.0, &nodes_memory_labels.1),
            false,
        ))
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
        acc + parse_memory(&node.status.capacity.memory).unwrap_or(0.)
    }));
    let nuse = convert_memory(metrics.iter().fold(0., |acc, node| {
        acc + parse_memory(&node.usage.memory).unwrap_or(0.)
    }));
    ((ncap.0, nuse.0), (ncap.1, nuse.1))
}
