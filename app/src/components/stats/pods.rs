use api::metrics as metrics_api;
use api::workloads::pods as pods_api;
use domain::cluster::pod::Pod;
use domain::metrics::PodMetrics;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::shared::card::card_circle::card_circle;
use crate::components::stats::shared::{get_pods_cpu, get_pods_memory};

/// Namespace `None` means "all namespaces"; `node_name` scopes to a single
/// node's pods instead when given.
pub async fn render(
    cache: &CacheStore,
    namespace_name: Option<&str>,
    node_name: Option<&str>,
) -> Element {
    let pods = pods_api::get_pods(
        cache,
        namespace_name.map(ToString::to_string),
        node_name.map(ToString::to_string),
    )
    .await
    .unwrap_or_default();
    let pod_names = pods
        .iter()
        .map(|p| p.metadata.name.clone())
        .collect::<Vec<String>>();
    let pods_metrics = metrics_api::get_pods(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|pm| pod_names.contains(&pm.metadata.name))
        .collect::<Vec<PodMetrics>>();

    let pods_ready = get_pods_ready(&pods);
    let pods_cpu = get_pods_cpu(&pods, &pods_metrics);
    let (pods_memory_values, pods_memory_labels) = get_pods_memory(&pods, &pods_metrics);

    div()
        .class("card-container dcc-3")
        .child(card_circle(
            "Pods",
            "ready vs requested",
            pods_ready,
            ("", ""),
            true,
        ))
        .child(card_circle(
            "Pods CPU usage",
            "actual vs reserved",
            pods_cpu,
            ("", ""),
            false,
        ))
        .child(card_circle(
            "Pods Memory usage",
            "used vs available",
            pods_memory_values,
            (&pods_memory_labels.0, &pods_memory_labels.1),
            false,
        ))
}

fn get_pods_ready(pods: &[Pod]) -> (f64, f64) {
    let pcount = pods.len();
    let pready = pods
        .iter()
        .filter(|s| {
            s.status
                .conditions
                .iter()
                .any(|c| c.r#type == "Ready" && c.status == "True")
        })
        .count();
    (pcount as f64, pready as f64)
}
