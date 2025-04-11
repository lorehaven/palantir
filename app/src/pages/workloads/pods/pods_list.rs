use leptos::prelude::*;
use leptos::task::spawn_local;

use api::metrics as metrics_api;
use api::workloads::pods as pods_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use domain::metrics::PodMetrics;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::stats::pod_stats::{pod_cpu_actual, pod_cpu_limit, pod_cpu_request, pod_memory_actual, pod_memory_limit, pod_memory_request};

#[component]
pub fn PodsListComponent(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let pods = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(selected, prompt, pods));
    clear_page_effect(interval_handle);
    view(pods)
}

fn update_page(
    namespace_name: RwSignal<String>,
    pod_name: RwSignal<String>,
    pods: RwSignal<Vec<Vec<String>>>,
) {
    if namespace_name.is_disposed() || pod_name.is_disposed() { return; }
    let selected_value = namespace_name.get();
    let pod_name = pod_name.get();

    spawn_local(async move {
        let namespace_name = if selected_value == "All Namespaces" { None } else { Some(selected_value) };
        let pods_data = pods_api::get_pods(namespace_name, None).await
            .unwrap_or_default()
            .into_iter()
            .filter(|p| p.metadata.name.contains(&pod_name))
            .collect::<Vec<_>>();
        let pod_names = pods_data.iter().map(|p| p.metadata.name.clone()).collect::<Vec<String>>();
        let pods_metrics = metrics_api::get_pods().await.unwrap_or_default()
            .into_iter()
            .filter(|pm| pod_names.contains(&pm.metadata.name))
            .collect::<Vec<PodMetrics>>();

        let mut pods_vec = vec![];
        for pod in pods_data {
            let metrics = pods_metrics.clone().into_iter()
                .find(|p| p.metadata.name == pod.metadata.name)
                .unwrap_or_default();

            pods_vec.push(vec![
                "Pod".to_string(),
                pod.clone().metadata.namespace,
                pod.clone().metadata.name,
                pod_cpu_actual(&metrics),
                pod_cpu_request(&pod, &metrics),
                pod_cpu_limit(&pod, &metrics),
                pod_memory_actual(&metrics),
                pod_memory_request(&pod, &metrics),
                pod_memory_limit(&pod, &metrics),
            ]);
        }
        pods.set(pods_vec);
    });
}

fn view(
    replicas: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 3),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("CPU actual", TableColumnType::String, 1),
        TableColumn::new("CPU request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("CPU limit", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM actual", TableColumnType::String, 1),
        TableColumn::new("RAM request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM limit", TableColumnType::StringTwoLine, 1),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/workloads/:1/pods/";

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
