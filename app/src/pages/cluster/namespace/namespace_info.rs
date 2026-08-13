use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn fragment(cache: &CacheStore, namespace_name: &str) -> Element {
    let response = api::cluster::namespaces::get_namespaces_response(cache)
        .await
        .unwrap_or_default();
    let kind = if response.kind == "NamespaceList" {
        "Namespace".to_string()
    } else {
        response.kind
    };
    let namespace = response
        .items
        .into_iter()
        .find(|n| n.metadata.name == namespace_name)
        .unwrap_or_default();

    let data = vec![
        ("Name".to_string(), namespace.metadata.name.clone()),
        ("Kind".to_string(), kind),
        (
            "Created".to_string(),
            format_timestamp(
                namespace
                    .metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        (
            "Labels".to_string(),
            display::hashmap(namespace.metadata.labels),
        ),
        (
            "Annotations".to_string(),
            display::hashmap(namespace.metadata.annotations),
        ),
        ("Version".to_string(), namespace.metadata.resource_version),
        ("Status".to_string(), namespace.status.phase),
    ];

    resource_info_view(&data)
        .attr("id", "namespace-info")
        .attr(
            "hx-get",
            format!(
                "{}/cluster/namespaces/{namespace_name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}
