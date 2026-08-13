use api::metrics as metrics_api;
use api::workloads::{jobs as jobs_api, pods as pods_api};
use domain::cluster::pod::Pod;
use domain::workload::job::Job;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::events;
use crate::components::prelude::*;
use crate::components::stats::shared::{get_pods_cpu, get_pods_memory};
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;
use crate::utils::stats::pod_stats::{
    pod_cpu_actual, pod_cpu_limit, pod_cpu_request, pod_memory_actual, pod_memory_limit,
    pod_memory_request,
};

pub async fn render(cache: &CacheStore, current_path: &str, namespace: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/workloads/{namespace}/jobs/{name}",
        crate::base_path::ui_base()
    );
    let events_url = format!(
        "{}/workloads/{namespace}/jobs/{name}/events/fragment",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Workloads", namespace, "Job", name],
        current_path,
        div()
            .class("workloads-job main-page")
            .child(actions(
                "Job",
                vec![
                    edit_action(cache, "Job", Some(namespace), name).await,
                    delete_action("Job", Some(namespace), name, &confirm_url),
                ],
            ))
            .child(stats_fragment(cache, namespace, name).await)
            .child(info_fragment(cache, namespace, name).await)
            .child(container_fragment(cache, namespace, name).await)
            .child(pods_fragment(cache, namespace, name).await)
            .child(events::render(cache, "Job", namespace, name, &events_url).await),
    )
}

async fn find(cache: &CacheStore, namespace: &str, name: &str) -> Job {
    jobs_api::get_jobs(cache, Some(namespace.to_string()))
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|j| j.metadata.name == name)
        .unwrap_or_default()
}

async fn job_pods(cache: &CacheStore, namespace: &str, name: &str) -> Vec<Pod> {
    pods_api::get_pods(cache, Some(namespace.to_string()), None)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|p| {
            p.metadata
                .labels
                .get("job-name")
                .cloned()
                .unwrap_or_default()
                == name
        })
        .collect()
}

pub async fn stats_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let job = find(cache, namespace, name).await;
    let pods = job_pods(cache, namespace, name).await;
    let pod_metrics = metrics_api::get_pods(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|p| p.metadata.namespace == namespace)
        .collect::<Vec<_>>();

    let jobs_ready = format!("{} / {}", job.status.ready, job.status.succeeded);
    let pod_cpu_usage = get_pods_cpu(&pods, &pod_metrics);
    let (pod_memory_values, pod_memory_labels) = get_pods_memory(&pods, &pod_metrics);

    wrapper(
        "",
        div()
            .class("card-container dcc-3")
            .child(card_string("", "Active / Succeeded", &jobs_ready))
            .child(card_circle(
                "Pod CPU usage",
                "actual vs reserved",
                pod_cpu_usage,
                ("", ""),
                false,
            ))
            .child(card_circle(
                "Pod RAM usage",
                "actual vs reserved",
                pod_memory_values,
                (&pod_memory_labels.0, &pod_memory_labels.1),
                false,
            )),
    )
    .attr("id", "job-stats")
    .attr(
        "hx-get",
        format!(
            "{}/workloads/{namespace}/jobs/{name}/stats/fragment",
            crate::base_path::ui_base()
        ),
    )
    .attr("hx-trigger", "every 10s")
    .attr("hx-target", "this")
    .attr("hx-swap", "outerHTML")
}

pub async fn info_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let job = find(cache, namespace, name).await;
    let duration = job_duration(&job.status.start_time, &job.status.completion_time);

    let data = vec![
        ("Name".to_string(), job.metadata.name.clone()),
        ("Kind".to_string(), "Job".to_string()),
        ("Namespace".to_string(), job.metadata.namespace.clone()),
        (
            "Created".to_string(),
            format_timestamp(
                job.metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        ("Labels".to_string(), display::hashmap(job.metadata.labels)),
        (
            "Annotations".to_string(),
            display::hashmap(job.metadata.annotations),
        ),
        ("Version".to_string(), job.metadata.resource_version),
        ("Start Time".to_string(), job.status.start_time.clone()),
        (
            "Completion Time".to_string(),
            job.status.completion_time.clone(),
        ),
        ("Duration".to_string(), duration),
    ];

    resource_info_view(&data)
        .attr("id", "job-info")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/jobs/{name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn job_duration(start_time: &str, end_time: &str) -> String {
    let Ok(start) = start_time.parse::<chrono::DateTime<chrono::Utc>>() else {
        return "-".to_string();
    };
    let Ok(end) = end_time.parse::<chrono::DateTime<chrono::Utc>>() else {
        return "-".to_string();
    };

    let duration = end - start;
    let total_millis = duration.num_milliseconds();
    let hours = total_millis / 3_600_000;
    let minutes = (total_millis % 3_600_000) / 60_000;
    let seconds = (total_millis % 60_000) / 1_000;
    let millis = total_millis % 1_000;

    let mut parts = Vec::new();
    if hours != 0 {
        parts.push(format!("{hours}h"));
    }
    if minutes != 0 {
        parts.push(format!("{minutes}m"));
    }
    if seconds != 0 {
        parts.push(format!("{seconds}s"));
    }
    if millis != 0 {
        parts.push(format!("{millis}ms"));
    }

    if parts.is_empty() {
        "0ms".to_string()
    } else {
        parts.join(" ")
    }
}

pub async fn container_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let pod = job_pods(cache, namespace, name)
        .await
        .into_iter()
        .next()
        .unwrap_or_default();
    let container = pod.spec.containers.first().cloned().unwrap_or_default();
    let env = container
        .env
        .into_iter()
        .map(|e| format!("{}: {}", e.name, e.value))
        .collect::<Vec<String>>()
        .join("\n");

    let data = vec![
        ("Container".to_string(), container.name),
        ("Image".to_string(), container.image),
        ("Args".to_string(), container.args.join(" ")),
        ("Env".to_string(), env),
    ];

    resource_info_view(&data)
        .attr("id", "job-container")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/jobs/{name}/container/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

pub async fn pods_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let columns = pods_columns();
    let mut params = vec![String::new(); columns.len()];
    params[1] = format!("/workloads/{namespace}/pods/");
    let styles = vec![String::new(); columns.len()];

    let pods = job_pods(cache, namespace, name).await;
    let pod_names = pods
        .iter()
        .map(|p| p.metadata.name.clone())
        .collect::<Vec<String>>();
    let pods_metrics = metrics_api::get_pods(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|pm| pod_names.contains(&pm.metadata.name))
        .collect::<Vec<_>>();

    let mut rows = pods
        .into_iter()
        .map(|r| {
            let metrics = pods_metrics
                .iter()
                .find(|p| p.metadata.name == r.metadata.name)
                .cloned()
                .unwrap_or_default();
            vec![
                "Pod".to_string(),
                r.metadata.name.clone(),
                pod_cpu_actual(&metrics),
                pod_cpu_request(&r, &metrics),
                pod_cpu_limit(&r, &metrics),
                pod_memory_actual(&metrics),
                pod_memory_request(&r, &metrics),
                pod_memory_limit(&r, &metrics),
            ]
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a[1].cmp(&b[1]));
    let rows = parse_table_rows(&columns, rows, &styles, &params);

    data_list_view(&columns, &rows)
        .attr("id", "job-pods")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/jobs/{name}/pods/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn pods_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("CPU actual", TableColumnType::String, 1),
        TableColumn::new("CPU request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("CPU limit", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM actual", TableColumnType::String, 1),
        TableColumn::new("RAM request", TableColumnType::StringTwoLine, 1),
        TableColumn::new("RAM limit", TableColumnType::StringTwoLine, 1),
    ]
}
