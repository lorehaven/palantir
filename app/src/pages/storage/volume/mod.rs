use api::storage::volumes as volumes_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, name: &str) -> String {
    let confirm_url = format!("{}/storage/volumes/{name}", crate::base_path::ui_base());

    crate::shell::page(
        &["Storage", "Persistent Volumes", name],
        current_path,
        div()
            .class("storage-volume main-page")
            .child(actions(
                "PersistentVolume",
                vec![
                    edit_action(cache, "PersistentVolume", None, name).await,
                    delete_action("PersistentVolume", None, name, &confirm_url),
                ],
            ))
            .child(fragment(cache, name).await),
    )
}

pub async fn fragment(cache: &CacheStore, name: &str) -> Element {
    let volume = volumes_api::get_volumes(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|v| v.metadata.name == name)
        .unwrap_or_default();

    let data = vec![
        ("Name".to_string(), volume.metadata.name.clone()),
        ("Kind".to_string(), "PersistentVolume".to_string()),
        (
            "Created".to_string(),
            format_timestamp(
                volume
                    .metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        (
            "Labels".to_string(),
            display::hashmap(volume.metadata.labels),
        ),
        (
            "Annotations".to_string(),
            display::hashmap(volume.metadata.annotations),
        ),
        ("Version".to_string(), volume.metadata.resource_version),
        ("Status".to_string(), volume.status.phase),
        ("Class".to_string(), String::new()),
        (
            "Claim".to_string(),
            format!(
                "{}/{}",
                volume.spec.claim_ref.namespace, volume.spec.claim_ref.name
            ),
        ),
        (
            "Access Modes".to_string(),
            volume.spec.access_mode.join("\n"),
        ),
        ("Capacity".to_string(), volume.spec.capacity.storage),
        (
            "Reclaim Policy".to_string(),
            volume.spec.persistent_volume_reclaim_policy,
        ),
        ("Local Path".to_string(), String::new()),
    ];

    resource_info_view(&data)
        .attr("id", "volume-info")
        .attr(
            "hx-get",
            format!(
                "{}/storage/volumes/{name}/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}
