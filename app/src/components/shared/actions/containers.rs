use domain::cluster::pod::Container;
use quench_cache::CacheStore;
use quench_web::prelude::*;

/// The container names for a single pod, fetched from its raw resource JSON
/// (there's no dedicated "list containers" API - same source the old Leptos
/// version used).
pub async fn container_names(cache: &CacheStore, namespace: &str, name: &str) -> Vec<String> {
    let json = api::resource::get(
        cache,
        "Pod",
        Some(namespace.to_string()),
        Some(name.to_string()),
    )
    .await
    .unwrap_or_default();
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&json) else {
        return Vec::new();
    };
    value["spec"]["containers"]
        .as_array()
        .map(|containers| {
            containers
                .iter()
                .filter_map(|c| serde_json::from_value::<Container>(c.clone()).ok())
                .map(|c| c.name)
                .collect()
        })
        .unwrap_or_default()
}

/// Like `namespaces::namespaces_filter_action`, carries no htmx wiring of
/// its own.
pub fn containers_filter_action(id: &str, containers: &[String], selected: &str) -> Element {
    containers.iter().fold(
        select()
            .attr("id", id)
            .attr("name", "container")
            .class("action containers-action"),
        |el, name| {
            let mut opt = option().text(name);
            if name == selected {
                opt = opt.attr("selected", "selected");
            }
            el.child(opt)
        },
    )
}
