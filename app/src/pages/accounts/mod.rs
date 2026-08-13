pub mod binding;
pub mod bindings;
pub mod clusterbinding;
pub mod clusterrole;
pub mod role;
pub mod roles;
pub mod secret;
pub mod secrets;
pub mod serviceaccount;

use api::accounts::serviceaccounts as accounts_api;
use domain::utils::time::time_until_now;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const FRAGMENT_ID: &str = "serviceaccounts-table";
const FILTER_ID: &str = "serviceaccounts-filter";
const NAMESPACE_ID: &str = "serviceaccounts-namespace";
const POLL_TRIGGER: &str =
    "every 10s, keyup changed delay:300ms from:#serviceaccounts-filter, change from:#serviceaccounts-namespace";
const INCLUDE: &str = "#serviceaccounts-filter, #serviceaccounts-namespace";

pub async fn render(
    cache: &CacheStore,
    current_path: &str,
    namespace: &str,
    filter: &str,
) -> String {
    let namespace_select = namespaces_filter_action(cache, NAMESPACE_ID, namespace).await;

    crate::shell::page(
        &["Service Accounts"],
        current_path,
        div()
            .class("service-accounts main-page")
            .child(actions(
                "ServiceAccounts",
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
            format!("{}/accounts/fragment", crate::base_path::ui_base()),
        )
        .attr("hx-trigger", POLL_TRIGGER)
        .attr("hx-include", INCLUDE)
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 1),
        TableColumn::new("Name", TableColumnType::Link, 2),
        TableColumn::new("Age", TableColumnType::String, 1),
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
    params[2] = "/accounts/:1/serviceaccounts/".to_string();

    let namespace_filter = if namespace.is_empty() || namespace == "All Namespaces" {
        None
    } else {
        Some(namespace.to_string())
    };

    let mut list = accounts_api::get_serviceaccounts(cache, namespace_filter)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|n| {
            n.metadata
                .name
                .to_lowercase()
                .contains(&filter.to_lowercase())
        })
        .map(|sa| {
            vec![
                "ServiceAccount".to_string(),
                sa.metadata.namespace.clone(),
                sa.metadata.name.clone(),
                time_until_now(
                    sa.metadata
                        .creation_timestamp
                        .as_deref()
                        .unwrap_or_default(),
                ),
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    parse_table_rows(columns, list, &styles, &params)
}
