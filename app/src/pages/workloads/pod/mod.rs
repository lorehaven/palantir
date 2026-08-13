use api::metrics as metrics_api;
use api::workloads::pods as pods_api;
use domain::cluster::pod::Pod;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::events;
use crate::components::prelude::*;
use crate::components::stats::shared::{get_pods_cpu, get_pods_memory};
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, namespace: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/workloads/{namespace}/pods/{name}",
        crate::base_path::ui_base()
    );
    let events_url = format!(
        "{}/workloads/{namespace}/pods/{name}/events/fragment",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Workloads", namespace, "Pod", name],
        current_path,
        div()
            .class("workloads-pod main-page")
            .child(actions(
                "Pod",
                vec![
                    logs_action(namespace, name),
                    exec_action(namespace, name),
                    edit_action(cache, "Pod", Some(namespace), name).await,
                    delete_action("Pod", Some(namespace), name, &confirm_url),
                ],
            ))
            .child(stats_fragment(cache, namespace, name).await)
            .child(info_fragment(cache, namespace, name).await)
            .child(container_fragment(cache, namespace, name).await)
            .child(events::render(cache, "Pod", namespace, name, &events_url).await),
    )
}

async fn find(cache: &CacheStore, namespace: &str, name: &str) -> Pod {
    pods_api::get_pods(cache, Some(namespace.to_string()), None)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|p| p.metadata.name == name)
        .unwrap_or_default()
}

pub async fn stats_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let pod = find(cache, namespace, name).await;
    let pod_metrics = metrics_api::get_pods(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|p| p.metadata.namespace == namespace)
        .collect::<Vec<_>>();

    let pod_cpu_usage = get_pods_cpu(std::slice::from_ref(&pod), &pod_metrics);
    let (pod_memory_values, pod_memory_labels) =
        get_pods_memory(std::slice::from_ref(&pod), &pod_metrics);

    wrapper(
        "",
        div()
            .class("card-container dcc-2")
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
    .attr("id", "pod-stats")
    .attr(
        "hx-get",
        format!(
            "{}/workloads/{namespace}/pods/{name}/stats/fragment",
            crate::base_path::ui_base()
        ),
    )
    .attr("hx-trigger", "every 10s")
    .attr("hx-target", "this")
    .attr("hx-swap", "outerHTML")
}

pub async fn info_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let pod = find(cache, namespace, name).await;
    let owned_by = pod
        .metadata
        .owner_references
        .into_iter()
        .map(|or| format!("{namespace}/{}/{}", or.kind.to_lowercase(), or.name))
        .collect::<Vec<String>>()
        .join("\n");
    let conditions = pod
        .status
        .conditions
        .into_iter()
        .map(|c| format!("{} • {}", c.r#type, c.status))
        .collect::<Vec<String>>()
        .join("\n");

    let data = vec![
        ("Name".to_string(), pod.metadata.name.clone()),
        ("Kind".to_string(), "Pod".to_string()),
        ("Namespace".to_string(), pod.metadata.namespace.clone()),
        (
            "Created".to_string(),
            format_timestamp(
                pod.metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        ("Labels".to_string(), display::hashmap(pod.metadata.labels)),
        (
            "Annotations".to_string(),
            display::hashmap(pod.metadata.annotations),
        ),
        ("Version".to_string(), pod.metadata.resource_version),
        ("Owned By".to_string(), owned_by),
        ("Host IP".to_string(), pod.status.host_ip),
        ("Pod IP".to_string(), pod.status.pod_ip.unwrap_or_default()),
        ("QOS".to_string(), pod.status.qos_class),
        ("Phase".to_string(), pod.status.phase),
        ("Conditions".to_string(), conditions),
        ("Node Name".to_string(), pod.spec.node_name),
        ("Selector".to_string(), display::hashmap(pod.spec.selector)),
    ];

    resource_info_view(&data)
        .attr("id", "pod-info")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/pods/{name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

pub async fn container_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let pod = find(cache, namespace, name).await;
    let container = pod.spec.containers.first().cloned().unwrap_or_default();

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
        .map(|e| format!("{}: {}", e.name, e.value))
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
        .attr("id", "pod-container")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/pods/{name}/container/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}
