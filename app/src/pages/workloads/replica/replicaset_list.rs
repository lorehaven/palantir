use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::metrics as metrics_api;
use crate::api::workloads::pods as pods_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::domain::metrics::PodMetrics;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::stats::pod_stats::{pod_cpu_actual, pod_cpu_limit, pod_cpu_request, pod_memory_actual, pod_memory_limit, pod_memory_request};

#[component]
pub fn ReplicaSetListComponent(
    namespace_name: String,
    replicaset_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let replicaset_name = RwSignal::new(replicaset_name);
    let replicas = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(namespace_name, replicaset_name, replicas));
    clear_page_effect(interval_handle);
    view(namespace_name, replicas)
}

fn update_page(
    namespace_name: RwSignal<String>,
    replicaset_name: RwSignal<String>,
    replicas: RwSignal<Vec<Vec<String>>>,
) {
    spawn_local(async move {
        if namespace_name.is_disposed() || replicaset_name.is_disposed() { return; }

        let pods = pods_api::get_pods_by_namespace_name(namespace_name.get_untracked()).await.unwrap_or_default()
            .into_iter()
            .filter(|p| p.metadata.name.contains(&replicaset_name.get_untracked()))
            .collect::<Vec<_>>();
        let pod_names = pods.iter().map(|p| p.metadata.name.clone()).collect::<Vec<String>>();
        let pods_metrics = metrics_api::get_pods().await.unwrap_or_default()
            .into_iter()
            .filter(|pm| pod_names.contains(&pm.metadata.name))
            .collect::<Vec<PodMetrics>>();

        let mut pods_vec = vec![];
        for pod in pods {
            let metrics = pods_metrics.clone().into_iter()
                .find(|p| p.metadata.name == pod.metadata.name)
                .unwrap_or_default();

            pods_vec.push(vec![
                "Pod".to_string(),
                pod.clone().metadata.name,
                pod_cpu_actual(&metrics),
                pod_cpu_request(&pod, &metrics),
                pod_cpu_limit(&pod, &metrics),
                pod_memory_actual(&metrics),
                pod_memory_request(&pod, &metrics),
                pod_memory_limit(&pod, &metrics),
            ]);
        }
        replicas.set(pods_vec);
    });
}

fn view(
    namespace_name: RwSignal<String>,
    replicas: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("CPU actual", TableColumnType::String, 1),
        TableColumn::new("CPU request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("CPU limit", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM actual", TableColumnType::String, 1),
        TableColumn::new("RAM request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM limit", TableColumnType::StringTwoLine, 1),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = format!("/workloads/{}/pods/", namespace_name.get_untracked());

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=replicas.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
