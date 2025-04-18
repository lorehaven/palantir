use api::metrics as metrics_api;
use api::workloads::{pods as pods_api, deployments as deployments_api};
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::components::stats::shared::{get_pods_cpu, get_pods_memory};
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn DeploymentStatsComponent(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let replicas_ready = RwSignal::new((0., 0.));
    let pod_cpu_usage = RwSignal::new((0., 0.));
    let pod_memory_values = RwSignal::new((0., 0.));
    let pod_memory_labels = RwSignal::new((String::new(), String::new()));

    let interval_handle = update_page_effect(10_000, move || {
        update_page(
            namespace_name,
            resource_name,
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
    resource_name: RwSignal<String>,
    replicas_ready: RwSignal<(f64, f64)>,
    pod_cpu_usage: RwSignal<(f64, f64)>,
    pod_memory_values: RwSignal<(f64, f64)>,
    pod_memory_labels: RwSignal<(String, String)>,
) {
    if namespace_name.is_disposed() || resource_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let resource_name = resource_name.get();

    spawn_local(async move {
        let namespace_name = if namespace_name.clone() == "All Namespaces" {
            None
        } else {
            Some(namespace_name.clone())
        };
        let replica = deployments_api::get_deployments(namespace_name.clone())
            .await
            .unwrap_or_default()
            .into_iter()
            .find(|r| r.metadata.name == resource_name)
            .unwrap_or_default();
        let deployment_pods = pods_api::get_pods(namespace_name.clone(), None)
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|p| p.metadata.name.contains(&resource_name))
            .collect::<Vec<_>>();
        replicas_ready.set((
            replica.status.ready_replicas as f64,
            replica.status.replicas as f64,
        ));

        let pod_metrics = metrics_api::get_pods()
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|p| p.metadata.namespace == namespace_name.clone().unwrap_or_default())
            .collect::<Vec<_>>();

        pod_cpu_usage.set(get_pods_cpu(&deployment_pods, &pod_metrics));
        let pods_memory = get_pods_memory(&deployment_pods, &pod_metrics);
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
