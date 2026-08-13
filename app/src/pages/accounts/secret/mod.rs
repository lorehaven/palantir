use api::accounts::secrets as secrets_api;
use domain::account::secret::Secret;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, namespace: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/accounts/{namespace}/secrets/{name}",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Accounts", namespace, "Secrets", name],
        current_path,
        div()
            .class("accounts-secret main-page")
            .child(actions(
                "Secret",
                vec![
                    edit_action(cache, "Secret", Some(namespace), name).await,
                    delete_action("Secret", Some(namespace), name, &confirm_url),
                ],
            ))
            .child(info_fragment(cache, namespace, name).await)
            .child(data_fragment(cache, namespace, name).await),
    )
}

async fn find_secret(cache: &CacheStore, namespace: &str, name: &str) -> Secret {
    secrets_api::get_secrets(cache, Some(namespace.to_string()))
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|s| s.metadata.name == name)
        .unwrap_or_default()
}

pub async fn info_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let secret = find_secret(cache, namespace, name).await;

    let data = vec![
        ("Name".to_string(), secret.metadata.name.clone()),
        ("Kind".to_string(), "Secret".to_string()),
        ("Namespace".to_string(), secret.metadata.namespace.clone()),
        (
            "Created".to_string(),
            format_timestamp(
                secret
                    .metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        ("Version".to_string(), secret.metadata.resource_version),
        ("Type".to_string(), secret.r#type),
    ];

    resource_info_view(&data)
        .attr("id", "secret-info")
        .attr(
            "hx-get",
            format!(
                "{}/accounts/{namespace}/secrets/{name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

pub async fn data_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let secret = find_secret(cache, namespace, name).await;
    let data = secret
        .data
        .into_iter()
        .map(|(k, v)| (k, v.replace('\n', " ")))
        .collect::<Vec<_>>();

    let mut data = data;
    data.sort_by(|a, b| a.0.cmp(&b.0));

    obscured_resource_info_view(&data)
        .attr("id", "secret-data")
        .attr(
            "hx-get",
            format!(
                "{}/accounts/{namespace}/secrets/{name}/data/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}
