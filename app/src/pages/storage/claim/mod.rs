use api::storage::claims as claims_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, namespace: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/storage/{namespace}/claims/{name}",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Storage", namespace, "Persistent Volume Claims", name],
        current_path,
        div()
            .class("storage-claim main-page")
            .child(actions(
                "PersistentVolumeClaim",
                vec![
                    edit_action(cache, "PersistentVolumeClaim", Some(namespace), name).await,
                    delete_action("PersistentVolumeClaim", Some(namespace), name, &confirm_url),
                ],
            ))
            .child(fragment(cache, namespace, name).await),
    )
}

pub async fn fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let claim = claims_api::get_claims(cache, Some(namespace.to_string()))
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|c| c.metadata.name == name)
        .unwrap_or_default();

    let data = vec![
        ("Name".to_string(), claim.metadata.name.clone()),
        ("Kind".to_string(), "PersistentVolumeClaim".to_string()),
        ("Namespace".to_string(), claim.metadata.namespace.clone()),
        (
            "Created".to_string(),
            format_timestamp(
                claim
                    .metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        (
            "Labels".to_string(),
            display::hashmap(claim.metadata.labels),
        ),
        (
            "Annotations".to_string(),
            display::hashmap(claim.metadata.annotations),
        ),
        ("Version".to_string(), claim.metadata.resource_version),
        ("Status".to_string(), claim.status.phase),
        ("Class".to_string(), String::new()),
        ("Volume".to_string(), claim.spec.volume_name),
        ("Modes".to_string(), claim.spec.access_modes.join("\n")),
        (
            "Capacity".to_string(),
            claim.spec.resources.requests.storage,
        ),
    ];

    resource_info_view(&data)
        .attr("id", "claim-info")
        .attr(
            "hx-get",
            format!(
                "{}/storage/{namespace}/claims/{name}/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}
