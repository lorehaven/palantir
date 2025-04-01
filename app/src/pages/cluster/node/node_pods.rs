use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::metrics as metrics_api;
use crate::api::pods as pods_api;
use crate::components::prelude::*;
use crate::domain::metrics::PodMetrics;
use crate::domain::pod::Pod;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::time_until_now;
use crate::pages::utils::stats::{convert_memory, parse_memory, parse_pod_cpu};

#[component]
pub fn NodePodsComponent(
    node_name: String,
) -> impl IntoView {
    let node_name = RwSignal::new(node_name);
    let pods = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(node_name, pods));
    clear_page_effect(interval_handle);

    view(pods)
}

fn update_page(
    node_name: RwSignal<String>,
    pods: RwSignal<Vec<Vec<String>>>,
) {
    spawn_local(async move {
        if node_name.is_disposed() { return; }

        let mut pods_data = pods_api::get_pods_by_node_name(node_name.get_untracked()).await
            .unwrap_or_default();
        pods_data.sort_by(|a, b| a.metadata.name.cmp(&b.metadata.name));
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
                pod.clone().metadata.name,
                pod.clone().metadata.namespace,
                time_until_now(&pod.metadata.creation_timestamp),
                pod.clone().status.container_statuses.iter().map(|c| c.restart_count).sum::<i32>().to_string(),
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
    pods: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Namespace", TableColumnType::String, 2),
        TableColumn::new("Age", TableColumnType::String, 1),
        TableColumn::new("Restarts", TableColumnType::String, 1),
        TableColumn::new("CPU actual", TableColumnType::String, 1),
        TableColumn::new("CPU request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("CPU limit", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM actual", TableColumnType::String, 1),
        TableColumn::new("RAM request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM limit", TableColumnType::StringTwoLine, 1),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/workloads/:2/pods/";

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=pods.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}

fn pod_cpu_actual(metrics: &PodMetrics) -> String {
    let usage = parse_pod_cpu_actual_f64(metrics);
    if usage == 0. { "0m".to_string() }
    else { format!("{:.2}m", parse_pod_cpu_actual_f64(metrics) / 1_000_000.) }
}

fn pod_cpu_request(pod: &Pod, metrics: &PodMetrics) -> String {
    let request = pod.spec.containers.iter()
        .fold(0., |acc, c| acc + parse_pod_cpu(&c.resources.requests.cpu));
    let usage = parse_pod_cpu_actual_f64(metrics);
    let usage_percentage =
        if request == 0. { "-".to_string() }
        else { format!("{:.2}%", usage / 10_000_000. / request) };
    format!("{usage_percentage}\n{}m", request * 1000.)
}

fn pod_cpu_limit(pod: &Pod, metrics: &PodMetrics) -> String {
    let request = pod.spec.containers.iter()
        .fold(0., |acc, c| acc + parse_pod_cpu(&c.resources.limits.cpu));
    let usage = parse_pod_cpu_actual_f64(metrics);
    if request == 0. { "-".to_string() }
    else { format!("{:.2}%\n{}m", usage / 10_000_000. / request, request * 1000.) }
}

fn parse_pod_cpu_actual_f64(metrics: &PodMetrics) -> f64 {
    metrics
        .containers.iter()
        .fold(0., |acc, c| acc + c.usage.cpu.trim_end_matches('n').parse::<f64>().unwrap_or(0.))
}

fn pod_memory_actual(metrics: &PodMetrics) -> String {
    let (value, suffix) = convert_memory(parse_pod_memory_actual_f64(metrics));
    if value == 0. { "0Bi".to_string() }
    else { format!("{value:.2}{suffix}") }
}

fn pod_memory_request(pod: &Pod, metrics: &PodMetrics) -> String {
    let request = pod.spec.containers.iter()
        .filter(|c| !c.resources.requests.memory.is_empty())
        .fold(0., |acc, c| acc + parse_memory(&c.resources.requests.memory).unwrap_or_default());
    let actual = parse_pod_memory_actual_f64(metrics);
    let request_percentage = actual / request * 100.;
    let (request, suffix) = convert_memory(request);
    if request == 0. { "-".to_string() }
    else { format!("{request_percentage:.2}%\n{request}{suffix}") }
}

fn pod_memory_limit(pod: &Pod, metrics: &PodMetrics) -> String {
    let limit = pod.spec.containers.iter()
        .filter(|c| !c.resources.limits.memory.is_empty())
        .fold(0., |acc, c| acc + parse_memory(&c.resources.limits.memory).unwrap_or_default());
    let actual = parse_pod_memory_actual_f64(metrics);
    let limit_percentage = actual / limit * 100.;
    let (limit, suffix) = convert_memory(limit);
    if limit == 0. { "-".to_string() }
    else { format!("{limit_percentage:.2}%\n{limit}{suffix}") }
}

fn parse_pod_memory_actual_f64(metrics: &PodMetrics) -> f64 {
    metrics
        .containers.iter()
        .fold(0., |acc, c| acc + parse_memory(&c.usage.memory).unwrap_or_default())
}
