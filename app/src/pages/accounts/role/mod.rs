use api::accounts::roles as roles_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, namespace: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/accounts/{namespace}/roles/{name}",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Accounts", namespace, "Roles", name],
        current_path,
        div()
            .class("accounts-role main-page")
            .child(actions(
                "Role",
                vec![
                    edit_action(cache, "Role", Some(namespace), name).await,
                    delete_action("Role", Some(namespace), name, &confirm_url),
                ],
            ))
            .child(info_fragment(cache, namespace, name).await)
            .child(rules_fragment(cache, namespace, name).await),
    )
}

pub async fn info_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let role = roles_api::get_roles(cache, Some(namespace.to_string()))
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|r| r.metadata.name == name)
        .unwrap_or_default();

    let data = vec![
        ("Name".to_string(), role.metadata.name.clone()),
        ("Kind".to_string(), "Role".to_string()),
        (
            "Created".to_string(),
            format_timestamp(
                role.metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        ("Labels".to_string(), display::hashmap(role.metadata.labels)),
        (
            "Annotations".to_string(),
            display::hashmap(role.metadata.annotations),
        ),
        ("Version".to_string(), role.metadata.resource_version),
    ];

    resource_info_view(&data)
        .attr("id", "role-info")
        .attr(
            "hx-get",
            format!(
                "{}/accounts/{namespace}/roles/{name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

pub async fn rules_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let columns = rules_columns();
    let rows = roles_api::get_roles(cache, Some(namespace.to_string()))
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
                r.verbs.join("\n"),
                r.resource_names.join("\n"),
            ]
        })
        .collect::<Vec<_>>();

    let styles = vec![String::new(); columns.len()];
    let params = vec![String::new(); columns.len()];
    let rows = parse_table_rows(&columns, rows, &styles, &params);

    data_list_view(&columns, &rows)
        .attr("id", "role-rules")
        .attr(
            "hx-get",
            format!(
                "{}/accounts/{namespace}/roles/{name}/rules/fragment",
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
        TableColumn::new("Verbs", TableColumnType::StringList, 3),
        TableColumn::new("Names", TableColumnType::StringList, 3),
    ]
}
