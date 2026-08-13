use api::accounts::secrets as secrets_api;
use domain::utils::time::time_until_now;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const FRAGMENT_ID: &str = "secrets-table";
const FILTER_ID: &str = "secrets-filter";
const NAMESPACE_ID: &str = "secrets-namespace";
const POLL_TRIGGER: &str =
    "every 10s, keyup changed delay:300ms from:#secrets-filter, change from:#secrets-namespace";
const INCLUDE: &str = "#secrets-filter, #secrets-namespace";

pub async fn render(
    cache: &CacheStore,
    current_path: &str,
    namespace: &str,
    filter: &str,
) -> String {
    let namespace_select = namespaces_filter_action(cache, NAMESPACE_ID, namespace).await;

    crate::shell::page(
        &["Accounts", "Secrets"],
        current_path,
        div()
            .class("accounts-secrets main-page")
            .child(actions(
                "Secrets",
                vec![namespace_select, prompt_action(FILTER_ID, filter)],
            ))
            .child(fragment(cache, namespace, filter).await),
    )
}

pub async fn fragment(cache: &CacheStore, namespace: &str, filter: &str) -> Element {
    let columns = columns();
    let rows = rows(cache, &columns, namespace, filter).await;

    data_list_view(&columns, &rows)
        .attr("id", FRAGMENT_ID)
        .attr(
            "hx-get",
            format!("{}/accounts/secrets/fragment", crate::base_path::ui_base()),
        )
        .attr("hx-trigger", POLL_TRIGGER)
        .attr("hx-include", INCLUDE)
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 2),
        TableColumn::new("Name", TableColumnType::Link, 4),
        TableColumn::new("Age", TableColumnType::String, 1),
        TableColumn::new("Type", TableColumnType::String, 2),
    ]
}

async fn rows(
    cache: &CacheStore,
    columns: &[TableColumn],
    namespace: &str,
    filter: &str,
) -> Vec<TableRow> {
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/cluster/namespaces/".to_string();
    params[2] = "/accounts/:1/secrets/".to_string();

    let namespace_filter = if namespace.is_empty() || namespace == "All Namespaces" {
        None
    } else {
        Some(namespace.to_string())
    };

    let mut list = secrets_api::get_secrets(cache, namespace_filter)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|s| {
            s.metadata
                .name
                .to_lowercase()
                .contains(&filter.to_lowercase())
        })
        .map(|s| {
            vec![
                "Secret".to_string(),
                s.metadata.namespace.clone(),
                s.metadata.name.clone(),
                time_until_now(s.metadata.creation_timestamp.as_deref().unwrap_or_default()),
                s.r#type,
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    parse_table_rows(columns, list, &styles, &params)
}
