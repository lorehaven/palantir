use api::accounts::bindings as bindings_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/accounts/clusterrolebindings/{name}",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Accounts", "Cluster Role Bindings", name],
        current_path,
        div()
            .class("accounts-cluster-role-binding main-page")
            .child(actions(
                "ClusterRoleBinding",
                vec![
                    edit_action(cache, "ClusterRoleBinding", None, name).await,
                    delete_action("ClusterRoleBinding", None, name, &confirm_url),
                ],
            ))
            .child(info_fragment(cache, name).await)
            .child(subjects_fragment(cache, name).await),
    )
}

pub async fn info_fragment(cache: &CacheStore, name: &str) -> Element {
    let crb = bindings_api::get_clusterrolebindings(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|b| b.metadata.name == name)
        .unwrap_or_default();

    let data = vec![
        ("Name".to_string(), crb.metadata.name.clone()),
        ("Kind".to_string(), "ClusterRoleBinding".to_string()),
        (
            "Created".to_string(),
            format_timestamp(
                crb.metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        ("Labels".to_string(), display::hashmap(crb.metadata.labels)),
        (
            "Annotations".to_string(),
            display::hashmap(crb.metadata.annotations),
        ),
        ("Version".to_string(), crb.metadata.resource_version),
        ("Role".to_string(), crb.role_ref.name),
    ];

    resource_info_view(&data)
        .attr("id", "clusterbinding-info")
        .attr(
            "hx-get",
            format!(
                "{}/accounts/clusterrolebindings/{name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

pub async fn subjects_fragment(cache: &CacheStore, name: &str) -> Element {
    let columns = subjects_columns();
    let mut rows = bindings_api::get_clusterrolebindings(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|b| b.metadata.name == name)
        .unwrap_or_default()
        .subjects
        .into_iter()
        .map(|s| vec![s.kind, s.namespace, s.name, s.api_group])
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a[1].cmp(&b[1]));

    let styles = vec![String::new(); columns.len()];
    let params = vec![String::new(); columns.len()];
    let rows = parse_table_rows(&columns, rows, &styles, &params);

    data_list_view(&columns, &rows)
        .attr("id", "clusterbinding-subjects")
        .attr(
            "hx-get",
            format!(
                "{}/accounts/clusterrolebindings/{name}/subjects/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn subjects_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 3),
        TableColumn::new("Namespace", TableColumnType::String, 3),
        TableColumn::new("Name", TableColumnType::String, 3),
        TableColumn::new("Api Group", TableColumnType::String, 3),
    ]
}
