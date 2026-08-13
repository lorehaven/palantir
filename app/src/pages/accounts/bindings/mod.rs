use api::accounts::bindings as bindings_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const FRAGMENT_ID: &str = "bindings-table";
const FILTER_ID: &str = "bindings-filter";
const POLL_TRIGGER: &str = "every 10s, keyup changed delay:300ms from:#bindings-filter";

pub async fn render(cache: &CacheStore, current_path: &str, filter: &str) -> String {
    crate::shell::page(
        &["Accounts", "Role Bindings"],
        current_path,
        div()
            .class("accounts-bindings main-page")
            .child(actions(
                "RoleBindings",
                vec![prompt_action(FILTER_ID, filter)],
            ))
            .child(fragment(cache, filter).await),
    )
}

pub async fn fragment(cache: &CacheStore, filter: &str) -> Element {
    let columns = columns();
    let rows = rows(cache, &columns, filter).await;

    data_list_view(&columns, &rows)
        .attr("id", FRAGMENT_ID)
        .attr(
            "hx-get",
            format!("{}/accounts/bindings/fragment", crate::base_path::ui_base()),
        )
        .attr("hx-trigger", POLL_TRIGGER)
        .attr("hx-include", format!("#{FILTER_ID}"))
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Type", TableColumnType::String, 2),
        TableColumn::new("Namespace", TableColumnType::String, 2),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Age", TableColumnType::String, 1),
    ]
}

async fn rows(cache: &CacheStore, columns: &[TableColumn], filter: &str) -> Vec<TableRow> {
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/cluster/namespaces/".to_string();
    params[2] = "/accounts/:1/:0s/".to_string();

    let mut list = bindings_api::get_all_bindings(cache)
        .await
        .into_iter()
        .filter(|r| r.get_name().to_lowercase().contains(&filter.to_lowercase()))
        .map(|r| r.to_model())
        .map(|r| vec![r.r#type, r.namespace, r.name, r.age])
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[2].cmp(&b[2]));
    parse_table_rows(columns, list, &styles, &params)
}
