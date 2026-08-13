pub mod namespace_events;
pub mod namespace_info;
pub mod namespace_pods;

use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::pods as pods_stat;

pub async fn render(cache: &CacheStore, current_path: &str, namespace_name: &str) -> String {
    let confirm_url = format!(
        "{}/cluster/namespaces/{namespace_name}",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Cluster", "Namespaces", namespace_name],
        current_path,
        div()
            .class("cluster-namespace main-page")
            .child(actions(
                "Namespace",
                vec![delete_action(
                    "Namespace",
                    None,
                    namespace_name,
                    &confirm_url,
                )],
            ))
            .child(pods_stat::render(cache, Some(namespace_name), None).await)
            .child(namespace_info::fragment(cache, namespace_name).await)
            .child(namespace_pods::fragment(cache, namespace_name).await)
            .child(namespace_events::fragment(cache, namespace_name).await),
    )
}
