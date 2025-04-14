use api::metrics as metrics_api;
use api::workloads::pods as pods_api;
use domain::cluster::pod::Pod;
use domain::metrics::PodMetrics;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::components::stats::shared::{get_pods_cpu, get_pods_memory};
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn PodsStatComponent(
    #[prop(default = RwSignal::new("All Namespaces".to_string()))] namespace_name: RwSignal<String>,
    #[prop(default = RwSignal::new(String::new()))] node_name: RwSignal<String>,
    #[prop(default = true)] expandable: bool,
) -> impl IntoView {
    let pods_ready = RwSignal::new((0., 0.));
    let pods_cpu = RwSignal::new((0., 0.));
    let pods_memory_values = RwSignal::new((0., 0.));
    let pods_memory_labels = RwSignal::new((String::new(), String::new()));
    let expandable = RwSignal::new(expandable);

    let interval_handle = update_page_effect(10_000, move || {
        update_page(
            namespace_name,
            node_name,
            pods_ready,
            pods_cpu,
            pods_memory_values,
            pods_memory_labels,
        );
    });
    clear_page_effect(interval_handle);

    view(
        pods_ready,
        pods_cpu,
        pods_memory_values,
        pods_memory_labels,
        expandable,
    )
}

fn update_page(
    namespace_name: RwSignal<String>,
    node_name: RwSignal<String>,
    pods_ready: RwSignal<(f64, f64)>,
    pods_cpu: RwSignal<(f64, f64)>,
    pods_memory_values: RwSignal<(f64, f64)>,
    pods_memory_labels: RwSignal<(String, String)>,
) {
    if namespace_name.is_disposed() || node_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let node_name = node_name.get();

    spawn_local(async move {
        let namespace_name = if namespace_name == "All Namespaces" {
            None
        } else {
            Some(namespace_name)
        };
        let node_name = if node_name.is_empty() {
            None
        } else {
            Some(node_name)
        };
        let pods = pods_api::get_pods(namespace_name, node_name)
            .await
            .unwrap_or_default();
        let pod_names = pods
            .iter()
            .map(|p| p.metadata.name.clone())
            .collect::<Vec<String>>();
        let pods_metrics = metrics_api::get_pods()
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|pm| pod_names.contains(&pm.metadata.name))
            .collect::<Vec<PodMetrics>>();

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
    expandable: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Show
            when=move || expandable.get()
            fallback=move || view_internal(
                pods_ready,
                pods_cpu,
                pods_memory_values,
                pods_memory_labels
            )>
            <Expandable label="Pods" expanded=true>
                <ExpandableSlot slot>
                    {view_internal(
                        pods_ready,
                        pods_cpu,
                        pods_memory_values,
                        pods_memory_labels
                    )}
                </ExpandableSlot>
            </Expandable>
        </Show>
    }
}

fn view_internal(
    pods_ready: RwSignal<(f64, f64)>,
    pods_cpu: RwSignal<(f64, f64)>,
    pods_memory_values: RwSignal<(f64, f64)>,
    pods_memory_labels: RwSignal<(String, String)>,
) -> impl IntoView {
    view! {
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
    }
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
