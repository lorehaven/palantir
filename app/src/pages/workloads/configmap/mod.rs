use api::workloads::configmaps as configmaps_api;
use domain::workload::configmap::ConfigMap;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, namespace: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/workloads/{namespace}/configmaps/{name}",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Workloads", namespace, "ConfigMaps", name],
        current_path,
        div()
            .class("workloads-configmap main-page")
            .child(actions(
                "ConfigMap",
                vec![
                    edit_action(cache, "ConfigMap", Some(namespace), name).await,
                    delete_action("ConfigMap", Some(namespace), name, &confirm_url),
                ],
            ))
            .child(info_fragment(cache, namespace, name).await)
            .child(data_fragment(cache, namespace, name).await),
    )
}

async fn find(cache: &CacheStore, namespace: &str, name: &str) -> ConfigMap {
    configmaps_api::get_configmaps(cache, Some(namespace.to_string()))
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|c| c.metadata.name == name)
        .unwrap_or_default()
}

pub async fn info_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let configmap = find(cache, namespace, name).await;

    let data = vec![
        ("Name".to_string(), configmap.metadata.name.clone()),
        ("Kind".to_string(), "ConfigMap".to_string()),
        (
            "Namespace".to_string(),
            configmap.metadata.namespace.clone(),
        ),
        (
            "Created".to_string(),
            format_timestamp(
                configmap
                    .metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        ("Version".to_string(), configmap.metadata.resource_version),
    ];

    resource_info_view(&data)
        .attr("id", "configmap-info")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/configmaps/{name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

pub async fn data_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let configmap = find(cache, namespace, name).await;
    let mut data = configmap
        .data
        .into_iter()
        .map(|(k, v)| (k, v.replace('\n', " ")))
        .collect::<Vec<_>>();
    data.sort_by(|a, b| a.0.cmp(&b.0));

    resource_info_view(&data)
        .attr("id", "configmap-data")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/configmaps/{name}/data/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}
