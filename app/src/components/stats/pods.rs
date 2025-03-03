use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::domain::metrics::PodMetrics;
use crate::domain::pod::Pod;
use crate::components::prelude::*;
use crate::pages::utils::shared::effects::update_page_effect;
use crate::pages::utils::stats::{convert_memory, parse_memory};

#[component]
pub fn PodsStatComponent() -> impl IntoView {
    let pods_ready = RwSignal::new((0., 0.));
    let pods_cpu = RwSignal::new((0., 0.));
    let pods_memory_values = RwSignal::new((0., 0.));
    let pods_memory_labels = RwSignal::new((String::new(), String::new()));

    update_page_effect(5_000, move || update_page(
        pods_ready,
        pods_cpu,
        pods_memory_values,
        pods_memory_labels,));
    view(
        pods_ready,
        pods_cpu,
        pods_memory_values,
        pods_memory_labels,)
}

fn update_page(
    pods_ready: RwSignal<(f64, f64)>,
    pods_cpu: RwSignal<(f64, f64)>,
    pods_memory_values: RwSignal<(f64, f64)>,
    pods_memory_labels: RwSignal<(String, String)>,
) {
    spawn_local(async move {
        let pods = crate::api::pods::get_pods().await.unwrap_or_default();
        let pods_metrics = crate::api::metrics::get_pods().await.unwrap_or_default();
        pods_ready.set(get_pods_ready(&pods));
        pods_cpu.set(get_pods_cpu(&pods, &pods_metrics));
        let pods_memory = get_pods_memory(&pods, &pods_metrics);
        pods_memory_values.set(pods_memory.0);
        pods_memory_labels.set(pods_memory.1);
    });
}

fn view(
    pods_ready: RwSignal<(f64, f64)>,
    pods_cpu: RwSignal<(f64, f64)>,
    pods_memory_values: RwSignal<(f64, f64)>,
    pods_memory_labels: RwSignal<(String, String)>,
) -> impl IntoView {
    view! {
        <Expandable label="Pods" expanded=true>
            <ExpandableSlot slot>
                <div class="card-container dcc-3">
                    <CardCircle
                        label="Pods"
                        label_add="ready vs requested"
                        values=pods_ready.get() />
                    <CardCircle
                        label="Pods CPU usage"
                        label_add="actual vs reserved"
                        values=pods_cpu.get()
                        decimal=false />
                    <CardCircle
                        label="Pods Memory usage"
                        label_add="used vs available"
                        values=pods_memory_values.get()
                        value_labels=pods_memory_labels.get()
                        decimal=false />
                </div>
            </ExpandableSlot>
        </Expandable>
    }
}

fn get_pods_ready(pods: &[Pod]) -> (f64, f64) {
    let pcount = pods.len();
    let pready = pods.iter()
        .filter(|s| s.status.conditions.iter().any(|c| c.r#type == "Ready" && c.status == "True"))
        .count();
    (pcount as f64, pready as f64)
}

fn get_pods_cpu(pods: &[Pod], metrics: &[PodMetrics]) -> (f64, f64) {
    let pcap = pods.iter()
        .fold(0., |acc, p| acc + p.spec.containers.iter()
            .fold(0., |acc, c| acc + parse_pod_cpu(&c.resources.requests.cpu)));
    let puse = metrics.iter()
        .fold(0., |acc, p| acc + p.containers.iter()
            .fold(0., |acc, c| acc + c.usage.cpu.trim_end_matches('n').parse::<f64>().unwrap_or(0.)));
    (pcap, puse / 1_000_000_000.)
}

fn get_pods_memory(pods: &[Pod], metrics: &[PodMetrics]) -> ((f64, f64), (String, String)) {
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

fn parse_pod_cpu(request: &str) -> f64 {
    if request.ends_with("m") {
        let value: f64 = request.trim_end_matches("m").parse().unwrap_or(0.);
        value / 1000.
    } else {
        request.parse::<f64>().unwrap_or(0.)
    }
}
