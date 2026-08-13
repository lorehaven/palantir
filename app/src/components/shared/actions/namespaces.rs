use quench_cache::CacheStore;
use quench_web::prelude::*;

/// A namespace-scoping `<select>`.
///
/// Like `prompt::prompt_action`, carries no htmx wiring itself - whatever
/// polling fragment it scopes listens via `hx-trigger="... from:#{id}"` and
/// pulls its value in via `hx-include`.
pub async fn namespaces_filter_action(cache: &CacheStore, id: &str, selected: &str) -> Element {
    let mut names = api::cluster::namespaces::get_namespaces(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|n| n.metadata.name)
        .collect::<Vec<_>>();
    names.insert(0, "All Namespaces".to_string());

    names.into_iter().fold(
        select()
            .attr("id", id)
            .attr("name", "namespace")
            .class("action namespaces-action"),
        |el, name| {
            let mut opt = option().text(&name);
            if name == selected {
                opt = opt.attr("selected", "selected");
            }
            el.child(opt)
        },
    )
}
