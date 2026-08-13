use api::storage::storageclasses as storage_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, name: &str) -> String {
    let confirm_url = format!("{}/storageclasses/{name}", crate::base_path::ui_base());

    crate::shell::page(
        &["StorageClasses", name],
        current_path,
        div()
            .class("storageclass main-page")
            .child(actions(
                "StorageClass",
                vec![
                    edit_action(cache, "StorageClass", None, name).await,
                    delete_action("StorageClass", None, name, &confirm_url),
                ],
            ))
            .child(fragment(cache, name).await),
    )
}

pub async fn fragment(cache: &CacheStore, name: &str) -> Element {
    let storageclass = storage_api::get_storageclasses(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|sc| sc.metadata.name == name)
        .unwrap_or_default();

    let data = vec![
        ("Name".to_string(), storageclass.metadata.name.clone()),
        ("Kind".to_string(), "StorageClass".to_string()),
        (
            "Created".to_string(),
            format_timestamp(
                storageclass
                    .metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        (
            "Labels".to_string(),
            display::hashmap(storageclass.metadata.labels),
        ),
        (
            "Annotations".to_string(),
            display::hashmap(storageclass.metadata.annotations),
        ),
        (
            "Version".to_string(),
            storageclass.metadata.resource_version,
        ),
        ("Provisioner".to_string(), storageclass.provisioner),
        ("Policy".to_string(), storageclass.reclaim_policy),
        ("Mode".to_string(), storageclass.volume_binding_mode),
    ];

    resource_info_view(&data)
        .attr("id", "storageclass-info")
        .attr(
            "hx-get",
            format!(
                "{}/storageclasses/{name}/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}
