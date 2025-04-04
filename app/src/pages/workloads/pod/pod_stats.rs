use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::metrics as metrics_api;
use crate::api::workloads::pods as pods_api;
use crate::components::prelude::*;
use crate::components::stats::shared::{get_pods_cpu, get_pods_memory};
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn PodStatsComponent(
    namespace_name: String,
    pod_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let pod_name = RwSignal::new(pod_name);
    let pod_cpu_usage = RwSignal::new((0., 0.));
    let pod_memory_values = RwSignal::new((0., 0.));
    let pod_memory_labels = RwSignal::new((String::new(), String::new()));

    let interval_handle = update_page_effect(3_600_000, move || update_page(
        namespace_name,
        pod_name,
        pod_cpu_usage,
        pod_memory_values,
        pod_memory_labels,
    ));
    clear_page_effect(interval_handle);

    view(
        pod_cpu_usage,
        pod_memory_values,
        pod_memory_labels,
    )
}

fn update_page(
    namespace_name: RwSignal<String>,
    pod_name: RwSignal<String>,
    pod_cpu_usage: RwSignal<(f64, f64)>,
    pod_memory_values: RwSignal<(f64, f64)>,
    pod_memory_labels: RwSignal<(String, String)>,
) {
    spawn_local(async move {
        if namespace_name.is_disposed() || pod_name.is_disposed() { return; }

        let namespace_name = namespace_name.get_untracked();
        let pod_name = pod_name.get_untracked();
        let pod = pods_api::get_pods_by_namespace_name(namespace_name.clone()).await.unwrap_or_default()
            .into_iter()
            .find(|p| p.metadata.name == pod_name)
            .unwrap_or_default();

        let pod_metrics = metrics_api::get_pods().await.unwrap_or_default()
            .into_iter()
            .filter(|p| p.metadata.namespace == namespace_name && p.metadata.name.contains("drone-runner"))
            .collect::<Vec<_>>();

        pod_cpu_usage.set(get_pods_cpu(&[pod.clone()], &pod_metrics));
        let pods_memory = get_pods_memory(&[pod], &pod_metrics);
        pod_memory_values.set(pods_memory.0);
        pod_memory_labels.set(pods_memory.1);
    });
}

fn view(
    pod_cpu_usage: RwSignal<(f64, f64)>,
    pod_memory_values: RwSignal<(f64, f64)>,
    pod_memory_labels: RwSignal<(String, String)>,
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-3">
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
