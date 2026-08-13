pub mod configmap;
pub mod configmaps;
pub mod deployment;
pub mod ingress;
pub mod ingresses;
pub mod job;
pub mod pod;
pub mod pod_exec;
pub mod pod_logs;
pub mod pods;
pub mod replica;
pub mod replicas;
pub mod service;
pub mod services;

use api::workloads as workloads_api;
use api::workloads::pods as pods_api;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const FRAGMENT_ID: &str = "workloads-table";
const FILTER_ID: &str = "workloads-filter";
const NAMESPACE_ID: &str = "workloads-namespace";
const POLL_TRIGGER: &str =
    "every 10s, keyup changed delay:300ms from:#workloads-filter, change from:#workloads-namespace";
const INCLUDE: &str = "#workloads-filter, #workloads-namespace";

pub async fn render(
    cache: &CacheStore,
    current_path: &str,
    namespace: &str,
    filter: &str,
) -> String {
    let namespace_select = namespaces_filter_action(cache, NAMESPACE_ID, namespace).await;

    crate::shell::page(
        &["Workloads"],
        current_path,
        div()
            .class("workloads main-page")
            .child(actions(
                "Workloads",
                vec![namespace_select, prompt_action(FILTER_ID, filter)],
            ))
            .child(stats_fragment(cache, namespace).await)
            .child(fragment(cache, namespace, filter).await),
    )
}

pub async fn stats_fragment(cache: &CacheStore, namespace: &str) -> Element {
    let namespace_filter = namespace_filter(namespace);

    let workloads = workloads_api::get_workloads(cache, namespace_filter.clone()).await;
    let ready_workloads = workloads.iter().filter(|w| w.is_ready()).count();
    let workloads_ready = (workloads.len() as f64, ready_workloads as f64);

    let pods = pods_api::get_pods(cache, namespace_filter, None)
        .await
        .unwrap_or_default();
    let ready_pods = pods
        .iter()
        .filter(|p| {
            p.status
                .conditions
                .iter()
                .any(|pc| pc.r#type == "Ready" && pc.status == "True")
        })
        .count();
    let pods_ready = (pods.len() as f64, ready_pods as f64);

    wrapper(
        "",
        div()
            .class("card-container dcc-2")
            .child(card_circle(
                "Workloads",
                "ready vs requested",
                workloads_ready,
                ("", ""),
                true,
            ))
            .child(card_circle(
                "Pods",
                "ready vs requested",
                pods_ready,
                ("", ""),
                true,
            )),
    )
    .attr("id", "workloads-stats")
    .attr(
        "hx-get",
        format!("{}/workloads/stats/fragment", crate::base_path::ui_base()),
    )
    .attr(
        "hx-trigger",
        format!("every 10s, change from:#{NAMESPACE_ID}"),
    )
    .attr("hx-include", format!("#{NAMESPACE_ID}"))
    .attr("hx-target", "this")
    .attr("hx-swap", "outerHTML")
}

pub async fn fragment(cache: &CacheStore, namespace: &str, filter: &str) -> Element {
    let columns = columns();
    let rows = rows(cache, &columns, namespace, filter).await;

    data_list_view(&columns, &rows)
        .attr("id", FRAGMENT_ID)
        .attr(
            "hx-get",
            format!("{}/workloads/fragment", crate::base_path::ui_base()),
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
        TableColumn::new("Pods", TableColumnType::String, 3),
    ]
}

fn namespace_filter(namespace: &str) -> Option<String> {
    if namespace.is_empty() || namespace == "All Namespaces" {
        None
    } else {
        Some(namespace.to_string())
    }
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
    params[2] = "/workloads/:1/:0s/".to_string();

    let mut list = workloads_api::get_workloads(cache, namespace_filter(namespace))
        .await
        .into_iter()
        .filter(|w| w.get_name().to_lowercase().contains(&filter.to_lowercase()))
        .map(|w| w.to_model())
        .map(|w| vec![w.r#type, w.namespace, w.name, w.age, w.pods])
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    parse_table_rows(columns, list, &styles, &params)
}
