use crate::domain::metrics::{NodeMetrics, PodMetrics};
use crate::domain::node::Node;
use crate::domain::pod::Pod;

pub fn get_nodes_ready(nodes: &[Node]) -> (f64, f64) {
    let ncount = nodes.len();
    let nready = nodes.iter()
        .filter(|s| s.status.conditions.iter().any(|c| c.r#type == "Ready" && c.status == "True"))
        .count();
    (ncount as f64, nready as f64)
}

pub fn get_nodes_cpu(nodes: &[Node], metrics: &[NodeMetrics]) -> (f64, f64) {
    let ncap = nodes.iter().fold(0., |acc, node| acc + node.status.capacity.cpu.parse::<f64>().unwrap_or(0.));
    let nuse = metrics.iter().fold(0., |acc, node| acc + node.usage.cpu.trim_end_matches('n').parse::<f64>().unwrap_or(0.));
    (ncap, nuse / 1_000_000_000.)
}

pub fn get_nodes_memory(nodes: &[Node], metrics: &[NodeMetrics]) -> ((f64, f64), (String, String)) {
    let ncap = convert_memory(nodes.iter().fold(0., |acc, node| acc + parse_memory(&node.status.capacity.memory).unwrap_or(0.)));
    let nuse = convert_memory(metrics.iter().fold(0., |acc, node| acc + parse_memory(&node.usage.memory).unwrap_or(0.)));
    ((ncap.0, nuse.0), (ncap.1, nuse.1))
}

pub fn get_pods_ready(pods: &[Pod]) -> (f64, f64) {
    let pcount = pods.len();
    let pready = pods.iter()
        .filter(|s| s.status.conditions.iter().any(|c| c.r#type == "Ready" && c.status == "True"))
        .count();
    (pcount as f64, pready as f64)
}

pub fn get_pods_cpu(pods: &[Pod], metrics: &[PodMetrics]) -> (f64, f64) {
    let pcap = pods.iter()
        .fold(0., |acc, p| acc + p.spec.containers.iter()
            .fold(0., |acc, c| acc + parse_pod_cpu(&c.resources.requests.cpu)));
    let puse = metrics.iter()
        .fold(0., |acc, p| acc + p.containers.iter()
            .fold(0., |acc, c| acc + c.usage.cpu.trim_end_matches('n').parse::<f64>().unwrap_or(0.)));
    (pcap, puse / 1_000_000_000.)
}

pub fn get_pods_memory(pods: &[Pod], metrics: &[PodMetrics]) -> ((f64, f64), (String, String)) {
    let pcap = pods.iter()
        .fold(0., |acc, p| acc + p.spec.containers.iter()
            .filter(|c| !c.resources.requests.memory.is_empty())
            .fold(0., |acc, c| acc + parse_memory(&c.resources.requests.memory).unwrap_or_default()));
    let pcap = convert_memory(pcap);
    let puse = metrics.iter()
        .fold(0., |acc, p| acc + p.containers.iter()
            .fold(0., |acc, c| acc + parse_memory(&c.usage.memory).unwrap_or_default()));
    let puse = convert_memory(puse);
    ((pcap.0, puse.0), (pcap.1, puse.1))
}

fn parse_memory(memory_str: &str) -> Option<f64> {
    let (num_str, unit_str) = memory_str.split_at(memory_str.len() - 2);
    let num: f64 = num_str.parse().ok()?;
    match unit_str {
        "Ki" => Some(num),
        "Mi" => Some(num * 1024.),
        "Gi" => Some(num * 1024. * 1024.),
        _ => None,
    }
}

fn convert_memory(bytes: f64) -> (f64, String) {
    let units = ["Ki", "Mi", "Gi", "Ti"];
    let mut unit_idx = 0;

    let mut memory_value = bytes;
    while memory_value >= 1024. && unit_idx < units.len() - 1 {
        memory_value /= 1024.;
        unit_idx += 1;
    }

    (memory_value, units[unit_idx].to_string())
}

fn parse_pod_cpu(request: &str) -> f64 {
    if request.ends_with("m") {
        let value: f64 = request.trim_end_matches("m").parse().unwrap_or(0.);
        value / 1000.
    } else {
        request.parse::<f64>().unwrap_or(0.)
    }
}
