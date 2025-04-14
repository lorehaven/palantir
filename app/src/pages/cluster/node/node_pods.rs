use api::metrics as metrics_api;
use api::workloads::pods as pods_api;
use domain::cluster::pod::Pod;
use domain::metrics::PodMetrics;
use domain::utils::time::time_until_now;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::stats::{convert_memory, parse_memory, parse_pod_cpu};

#[component]
pub fn NodePodsComponent(resource_name: RwSignal<String>) -> impl IntoView {
    let table_rows = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

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
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/workloads/:2/pods/".to_string();

    let columns_update = columns.clone();
    let interval_handle = update_page_effect(10_000, move || {
        update_page(
            columns_update.clone(),
            styles.clone(),
            params.clone(),
            table_rows,
            resource_name,
            loading,
        );
    });
    clear_page_effect(interval_handle);
    data_list_view(columns, table_rows, loading)
}

fn update_page(
    columns: Vec<TableColumn>,
    styles: Vec<String>,
    params: Vec<String>,
    table_rows: RwSignal<Vec<TableRow>>,
    resource_name: RwSignal<String>,
    loading: RwSignal<bool>,
) {
    if resource_name.is_disposed() {
        return;
    }
    let resource_name = resource_name.get();

    spawn_local(async move {
        let list = update_page_async(
            columns.clone(),
            styles.clone(),
            params.clone(),
            resource_name.clone(),
        )
        .await
        .unwrap_or_default();
        table_rows.set(list);
        loading.set(false);
    });
}

#[server]
async fn update_page_async(
    columns: Vec<TableColumn>,
    styles: Vec<String>,
    params: Vec<String>,
    resource_name: String,
) -> Result<Vec<TableRow>, ServerFnError> {
    let pods_data = pods_api::get_pods(None, Some(resource_name))
        .await
        .unwrap_or_default();
    let pod_names = pods_data
        .iter()
        .map(|p| p.metadata.name.clone())
        .collect::<Vec<String>>();
    let pods_metrics = metrics_api::get_pods()
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|pm| pod_names.contains(&pm.metadata.name))
        .collect::<Vec<PodMetrics>>();

    let mut list = pods_data
        .into_iter()
        .map(|r| {
            let metrics = pods_metrics
                .clone()
                .into_iter()
                .find(|pp| pp.metadata.name == r.metadata.name)
                .unwrap_or_default();

            vec![
                "Pod".to_string(),
                r.clone().metadata.name,
                r.clone().metadata.namespace,
                time_until_now(&r.clone().metadata.creation_timestamp.unwrap_or_default()),
                r.status
                    .container_statuses
                    .iter()
                    .map(|c| c.restart_count)
                    .sum::<i32>()
                    .to_string(),
                pod_cpu_actual(&metrics),
                pod_cpu_request(&r, &metrics),
                pod_cpu_limit(&r, &metrics),
                pod_memory_actual(&metrics),
                pod_memory_request(&r, &metrics),
                pod_memory_limit(&r, &metrics),
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    Ok(parse_table_rows(columns, list, styles, params))
}

fn pod_cpu_actual(metrics: &PodMetrics) -> String {
    let usage = parse_pod_cpu_actual_f64(metrics);
    if usage == 0. {
        "0m".to_string()
    } else {
        format!("{:.2}m", parse_pod_cpu_actual_f64(metrics) / 1_000_000.)
    }
}

fn pod_cpu_request(pod: &Pod, metrics: &PodMetrics) -> String {
    let request = pod
        .spec
        .containers
        .iter()
        .fold(0., |acc, c| acc + parse_pod_cpu(&c.resources.requests.cpu));
    let usage = parse_pod_cpu_actual_f64(metrics);
    let usage_percentage = if request == 0. {
        "-".to_string()
    } else {
        format!("{:.2}%", usage / 10_000_000. / request)
    };
    format!("{usage_percentage}\n{}m", request * 1000.)
}

fn pod_cpu_limit(pod: &Pod, metrics: &PodMetrics) -> String {
    let request = pod
        .spec
        .containers
        .iter()
        .fold(0., |acc, c| acc + parse_pod_cpu(&c.resources.limits.cpu));
    let usage = parse_pod_cpu_actual_f64(metrics);
    if request == 0. {
        "-".to_string()
    } else {
        format!(
            "{:.2}%\n{}m",
            usage / 10_000_000. / request,
            request * 1000.
        )
    }
}

fn parse_pod_cpu_actual_f64(metrics: &PodMetrics) -> f64 {
    metrics.containers.iter().fold(0., |acc, c| {
        acc + c
            .usage
            .cpu
            .trim_end_matches('n')
            .parse::<f64>()
            .unwrap_or(0.)
    })
}

fn pod_memory_actual(metrics: &PodMetrics) -> String {
    let (value, suffix) = convert_memory(parse_pod_memory_actual_f64(metrics));
    if value == 0. {
        "0Bi".to_string()
    } else {
        format!("{value:.2}{suffix}")
    }
}

fn pod_memory_request(pod: &Pod, metrics: &PodMetrics) -> String {
    let request = pod
        .spec
        .containers
        .iter()
        .filter(|c| !c.resources.requests.memory.is_empty())
        .fold(0., |acc, c| {
            acc + parse_memory(&c.resources.requests.memory).unwrap_or_default()
        });
    let actual = parse_pod_memory_actual_f64(metrics);
    let request_percentage = actual / request * 100.;
    let (request, suffix) = convert_memory(request);
    if request == 0. {
        "-".to_string()
    } else {
        format!("{request_percentage:.2}%\n{request}{suffix}")
    }
}

fn pod_memory_limit(pod: &Pod, metrics: &PodMetrics) -> String {
    let limit = pod
        .spec
        .containers
        .iter()
        .filter(|c| !c.resources.limits.memory.is_empty())
        .fold(0., |acc, c| {
            acc + parse_memory(&c.resources.limits.memory).unwrap_or_default()
        });
    let actual = parse_pod_memory_actual_f64(metrics);
    let limit_percentage = actual / limit * 100.;
    let (limit, suffix) = convert_memory(limit);
    if limit == 0. {
        "-".to_string()
    } else {
        format!("{limit_percentage:.2}%\n{limit}{suffix}")
    }
}

fn parse_pod_memory_actual_f64(metrics: &PodMetrics) -> f64 {
    metrics.containers.iter().fold(0., |acc, c| {
        acc + parse_memory(&c.usage.memory).unwrap_or_default()
    })
}
