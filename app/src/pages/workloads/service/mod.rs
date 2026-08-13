use api::workloads::services as services_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::events;
use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, namespace: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/workloads/{namespace}/services/{name}",
        crate::base_path::ui_base()
    );
    let events_url = format!(
        "{}/workloads/{namespace}/services/{name}/events/fragment",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Workloads", namespace, "Service", name],
        current_path,
        div()
            .class("workloads-service main-page")
            .child(actions(
                "Service",
                vec![
                    edit_action(cache, "Service", Some(namespace), name).await,
                    delete_action("Service", Some(namespace), name, &confirm_url),
                ],
            ))
            .child(info_fragment(cache, namespace, name).await)
            .child(events::render(cache, "Service", namespace, name, &events_url).await),
    )
}

pub async fn info_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let service = services_api::get_services(cache, None)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|n| n.metadata.namespace == namespace && n.metadata.name == name)
        .unwrap_or_default();

    let ports = service
        .spec
        .ports
        .into_iter()
        .map(|p| {
            let target_port = p
                .target_port
                .as_ref()
                .map_or_else(String::new, |tp| format!(" • {tp}"));
            let node_port = p
                .node_port
                .as_ref()
                .map_or_else(String::new, |tp| format!(" • {tp}"));
            format!(
                "{} • {}{target_port}{node_port} • {}",
                p.name,
                p.port.unwrap_or(0),
                p.protocol
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let data = vec![
        ("Name".to_string(), service.metadata.name.clone()),
        ("Kind".to_string(), "Service".to_string()),
        ("Namespace".to_string(), service.metadata.namespace.clone()),
        (
            "Created".to_string(),
            format_timestamp(
                service
                    .metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        (
            "Labels".to_string(),
            display::hashmap(service.metadata.labels),
        ),
        (
            "Annotations".to_string(),
            display::hashmap(service.metadata.annotations),
        ),
        ("Version".to_string(), service.metadata.resource_version),
        ("Cluster IP".to_string(), service.spec.cluster_ip),
        ("Type".to_string(), service.spec.r#type),
        ("Affinity".to_string(), service.spec.session_affinity),
        (
            "Selector".to_string(),
            display::hashmap(service.spec.selector),
        ),
        ("Ports".to_string(), ports),
    ];

    resource_info_view(&data)
        .attr("id", "service-info")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/services/{name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}
