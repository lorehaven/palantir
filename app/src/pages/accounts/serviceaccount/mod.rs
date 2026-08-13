use api::accounts::serviceaccounts as serviceaccounts_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, namespace: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/accounts/{namespace}/serviceaccounts/{name}",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["ServiceAccounts", namespace, name],
        current_path,
        div()
            .class("service-account main-page")
            .child(actions(
                "ServiceAccount",
                vec![
                    edit_action(cache, "ServiceAccount", Some(namespace), name).await,
                    delete_action("ServiceAccount", Some(namespace), name, &confirm_url),
                ],
            ))
            .child(fragment(cache, namespace, name).await),
    )
}

pub async fn fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let sa = serviceaccounts_api::get_serviceaccounts(cache, Some(namespace.to_string()))
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|sa| sa.metadata.name == name)
        .unwrap_or_default();

    let data = vec![
        ("Name".to_string(), sa.metadata.name.clone()),
        ("Kind".to_string(), "ServiceAccount".to_string()),
        ("Namespace".to_string(), sa.metadata.namespace.clone()),
        (
            "Created".to_string(),
            format_timestamp(
                sa.metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        ("Labels".to_string(), display::hashmap(sa.metadata.labels)),
        (
            "Annotations".to_string(),
            display::hashmap(sa.metadata.annotations),
        ),
        ("Version".to_string(), sa.metadata.resource_version),
        ("Secrets".to_string(), String::new()),
    ];

    resource_info_view(&data)
        .attr("id", "serviceaccount-info")
        .attr(
            "hx-get",
            format!(
                "{}/accounts/{namespace}/serviceaccounts/{name}/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}
