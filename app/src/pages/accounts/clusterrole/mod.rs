use api::accounts::roles as roles_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/accounts/clusterroles/{name}",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Accounts", "Cluster Roles", name],
        current_path,
        div()
            .class("accounts-cluster-role main-page")
            .child(actions(
                "ClusterRole",
                vec![
                    edit_action(cache, "ClusterRole", None, name).await,
                    delete_action("ClusterRole", None, name, &confirm_url),
                ],
            ))
            .child(info_fragment(cache, name).await)
            .child(rules_fragment(cache, name).await),
    )
}

pub async fn info_fragment(cache: &CacheStore, name: &str) -> Element {
    let cr = roles_api::get_clusterroles(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|r| r.metadata.name == name)
        .unwrap_or_default();

    let data = vec![
        ("Name".to_string(), cr.metadata.name.clone()),
        ("Kind".to_string(), "ClusterRole".to_string()),
        (
            "Created".to_string(),
            format_timestamp(
                cr.metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        ("Labels".to_string(), display::hashmap(cr.metadata.labels)),
        (
            "Annotations".to_string(),
            display::hashmap(cr.metadata.annotations),
        ),
        ("Version".to_string(), cr.metadata.resource_version),
    ];

    resource_info_view(&data)
        .attr("id", "clusterrole-info")
        .attr(
            "hx-get",
            format!(
                "{}/accounts/clusterroles/{name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

pub async fn rules_fragment(cache: &CacheStore, name: &str) -> Element {
    let columns = rules_columns();
    let rows = roles_api::get_clusterroles(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|r| r.metadata.name == name)
        .unwrap_or_default()
        .rules
        .into_iter()
        .map(|r| {
            vec![
                r.api_groups.join("\n"),
                r.resources.join("\n"),
                String::new(),
                r.verbs.join("\n"),
                r.resource_names.join("\n"),
            ]
        })
        .collect::<Vec<_>>();

    let styles = vec![String::new(); columns.len()];
    let params = vec![String::new(); columns.len()];
    let rows = parse_table_rows(&columns, rows, &styles, &params);

    data_list_view(&columns, &rows)
        .attr("id", "clusterrole-rules")
        .attr(
            "hx-get",
            format!(
                "{}/accounts/clusterroles/{name}/rules/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn rules_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Groups", TableColumnType::StringList, 3),
        TableColumn::new("Resources", TableColumnType::StringList, 3),
        TableColumn::new("Non Resource", TableColumnType::StringList, 3),
        TableColumn::new("Verbs", TableColumnType::StringList, 3),
        TableColumn::new("Names", TableColumnType::StringList, 3),
    ]
}
