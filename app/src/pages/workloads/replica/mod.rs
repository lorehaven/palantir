use api::metrics as metrics_api;
use api::workloads::{pods as pods_api, replicasets as replicasets_api};
use domain::workload::replicaset::ReplicaSet;
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
        "{}/workloads/{namespace}/replicasets/{name}",
        crate::base_path::ui_base()
    );
    let events_url = format!(
        "{}/workloads/{namespace}/replicasets/{name}/events/fragment",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Workloads", namespace, "Replica Set", name],
        current_path,
        div()
            .class("workloads-replicaset main-page")
            .child(actions(
                "ReplicaSet",
                vec![
                    scale_action(cache, "ReplicaSet", Some(namespace), name).await,
                    edit_action(cache, "ReplicaSet", Some(namespace), name).await,
                    delete_action("ReplicaSet", Some(namespace), name, &confirm_url),
                ],
            ))
            .child(stats_fragment(cache, namespace, name).await)
            .child(info_fragment(cache, namespace, name).await)
            .child(container_fragment(cache, namespace, name).await)
            .child(pods_fragment(cache, namespace, name).await)
            .child(events::render(cache, "ReplicaSet", namespace, name, &events_url).await),
    )
}

async fn find(cache: &CacheStore, namespace: &str, name: &str) -> ReplicaSet {
    replicasets_api::get_replicasets(cache, Some(namespace.to_string()))
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|r| r.metadata.name == name)
        .unwrap_or_default()
}

pub async fn stats_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let replicaset = find(cache, namespace, name).await;
    let pods = pods_api::get_pods(cache, Some(namespace.to_string()), None)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|p| p.metadata.name.contains(name))
        .collect::<Vec<_>>();
    let pod_metrics = metrics_api::get_pods(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|p| p.metadata.namespace == namespace)
        .collect::<Vec<_>>();

    let replicas_ready = (
        replicaset.status.ready_replicas as f64,
        replicaset.status.replicas as f64,
    );
    let pod_cpu_usage = get_pods_cpu(&pods, &pod_metrics);
    let (pod_memory_values, pod_memory_labels) = get_pods_memory(&pods, &pod_metrics);

    wrapper(
        "",
        div()
            .class("card-container dcc-3")
            .child(card_circle("Replicas", "", replicas_ready, ("", ""), true))
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
    .attr("id", "replicaset-stats")
    .attr(
        "hx-get",
        format!(
            "{}/workloads/{namespace}/replicasets/{name}/stats/fragment",
            crate::base_path::ui_base()
        ),
    )
    .attr("hx-trigger", "every 10s")
    .attr("hx-target", "this")
    .attr("hx-swap", "outerHTML")
}

pub async fn info_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let replicaset = find(cache, namespace, name).await;
    let owned_by = replicaset
        .metadata
        .owner_references
        .into_iter()
        .map(|or| format!("{}/{namespace}/{}", or.kind.to_lowercase(), or.name))
        .collect::<Vec<String>>()
        .join("\n");

    let data = vec![
        ("Name".to_string(), replicaset.metadata.name.clone()),
        ("Kind".to_string(), "ReplicaSet".to_string()),
        (
            "Namespace".to_string(),
            replicaset.metadata.namespace.clone(),
        ),
        (
            "Created".to_string(),
            format_timestamp(
                replicaset
                    .metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        (
            "Labels".to_string(),
            display::hashmap(replicaset.metadata.labels),
        ),
        (
            "Annotations".to_string(),
            display::hashmap(replicaset.metadata.annotations),
        ),
        ("Version".to_string(), replicaset.metadata.resource_version),
        ("Owned By".to_string(), owned_by),
    ];

    resource_info_view(&data)
        .attr("id", "replicaset-info")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/replicasets/{name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

pub async fn container_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let replicaset = find(cache, namespace, name).await;
    let container = replicaset
        .spec
        .template
        .spec
        .containers
        .first()
        .cloned()
        .unwrap_or_default();

    let ports = container
        .ports
        .into_iter()
        .map(|p| {
            let port_name = if p.name.is_empty() {
                String::new()
            } else {
                format!("{} • ", p.name)
            };
            format!("{port_name}{} • {}", p.container_port, p.protocol)
        })
        .collect::<Vec<String>>()
        .join("\n");
    let env = container
        .env
        .into_iter()
        .map(|e| format!("{}: {}", e.name, display::pretty_if_json(&e.value)))
        .collect::<Vec<String>>()
        .join("\n");

    let data = vec![
        ("Container".to_string(), container.name),
        ("Image".to_string(), container.image),
        ("Env".to_string(), env),
        ("Cpu Request".to_string(), container.resources.requests.cpu),
        (
            "Memory Request".to_string(),
            container.resources.requests.memory,
        ),
        ("Cpu Limit".to_string(), container.resources.limits.cpu),
        (
            "Memory Limit".to_string(),
            container.resources.limits.memory,
        ),
        ("Ports".to_string(), ports),
    ];

    resource_info_view(&data)
        .attr("id", "replicaset-container")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/replicasets/{name}/container/fragment",
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

    let pods = pods_api::get_pods(cache, Some(namespace.to_string()), None)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|p| p.metadata.name.contains(name))
        .collect::<Vec<_>>();
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
        .attr("id", "replicaset-pods")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/replicasets/{name}/pods/fragment",
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
