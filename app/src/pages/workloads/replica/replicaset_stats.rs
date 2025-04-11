use api::metrics as metrics_api;
use api::workloads::{pods as pods_api, replicasets as replicasets_api};
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::components::stats::shared::{get_pods_cpu, get_pods_memory};
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn ReplicaSetsStatsComponent(namespace_name: String, replicaset_name: String) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let replicaset_name = RwSignal::new(replicaset_name);
    let replicas_ready = RwSignal::new((0., 0.));
    let pod_cpu_usage = RwSignal::new((0., 0.));
    let pod_memory_values = RwSignal::new((0., 0.));
    let pod_memory_labels = RwSignal::new((String::new(), String::new()));

    let interval_handle = update_page_effect(3_600_000, move || {
        update_page(
            namespace_name,
            replicaset_name,
            replicas_ready,
            pod_cpu_usage,
            pod_memory_values,
            pod_memory_labels,
        );
    });
    clear_page_effect(interval_handle);

    view(
        replicas_ready,
        pod_cpu_usage,
        pod_memory_values,
        pod_memory_labels,
    )
}

fn update_page(
    namespace_name: RwSignal<String>,
    replicaset_name: RwSignal<String>,
    replicas_ready: RwSignal<(f64, f64)>,
    pod_cpu_usage: RwSignal<(f64, f64)>,
    pod_memory_values: RwSignal<(f64, f64)>,
    pod_memory_labels: RwSignal<(String, String)>,
) {
    if namespace_name.is_disposed() || replicaset_name.is_disposed() {
        return;
    }
    let selected_value = namespace_name.get();
    let replicaset_name = replicaset_name.get();

    spawn_local(async move {
        let namespace_name = if selected_value.clone() == "All Namespaces" {
            None
        } else {
            Some(selected_value.clone())
        };
        let replica = replicasets_api::get_replicasets(namespace_name.clone())
            .await
            .unwrap_or_default()
            .into_iter()
            .find(|r| r.metadata.name == replicaset_name)
            .unwrap_or_default();
        let replicaset_pods = pods_api::get_pods(namespace_name.clone(), None)
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|p| p.metadata.name.contains(&replicaset_name))
            .collect::<Vec<_>>();
        replicas_ready.set((
            replica.status.ready_replicas as f64,
            replica.status.replicas as f64,
        ));

        let pod_metrics = metrics_api::get_pods()
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|p| {
                p.metadata.namespace == selected_value && p.metadata.name.contains("drone-runner")
            })
            .collect::<Vec<_>>();

        pod_cpu_usage.set(get_pods_cpu(&replicaset_pods, &pod_metrics));
        let pods_memory = get_pods_memory(&replicaset_pods, &pod_metrics);
        pod_memory_values.set(pods_memory.0);
        pod_memory_labels.set(pods_memory.1);
    });
}

fn view(
    replicas_ready: RwSignal<(f64, f64)>,
    pod_cpu_usage: RwSignal<(f64, f64)>,
    pod_memory_values: RwSignal<(f64, f64)>,
    pod_memory_labels: RwSignal<(String, String)>,
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-3">
                    <CardCircle
                        label="Replicas"
                        values=replicas_ready.get() />
                    <CardCircle
                        label="Pod CPU usage"
                        label_add="actual vs reserved"
                        values=pod_cpu_usage.get()
                        decimal=false />
                    <CardCircle
                        label="Pod RAM usage"
                        label_add="actual vs reserved"
                        values=pod_memory_values.get()
                        value_labels=pod_memory_labels.get()
                        decimal=false />
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
