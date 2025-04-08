use crate::domain::metrics::PodMetrics;
use crate::domain::cluster::pod::Pod;
use crate::pages::utils::stats::{convert_memory, parse_memory, parse_pod_cpu};

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
