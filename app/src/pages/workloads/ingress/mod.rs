use api::workloads::ingresses as ingresses_api;
use domain::workload::ingress::Ingress;
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::time::format_timestamp;

pub async fn render(cache: &CacheStore, current_path: &str, namespace: &str, name: &str) -> String {
    let confirm_url = format!(
        "{}/workloads/{namespace}/ingresses/{name}",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Workloads", namespace, "Ingresses", name],
        current_path,
        div()
            .class("workloads-ingress main-page")
            .child(actions(
                "Ingress",
                vec![
                    edit_action(cache, "Ingress", Some(namespace), name).await,
                    delete_action("Ingress", Some(namespace), name, &confirm_url),
                ],
            ))
            .child(info_fragment(cache, namespace, name).await)
            .child(rules_fragment(cache, namespace, name).await),
    )
}

async fn find(cache: &CacheStore, namespace: &str, name: &str) -> Ingress {
    ingresses_api::get_ingresses(cache, Some(namespace.to_string()))
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|n| n.metadata.name == name)
        .unwrap_or_default()
}

pub async fn info_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let ingress = find(cache, namespace, name).await;

    let data = vec![
        ("Name".to_string(), ingress.metadata.name.clone()),
        ("Kind".to_string(), "Ingress".to_string()),
        ("Namespace".to_string(), ingress.metadata.namespace.clone()),
        (
            "Created".to_string(),
            format_timestamp(
                ingress
                    .metadata
                    .creation_timestamp
                    .as_deref()
                    .unwrap_or_default(),
                None,
            ),
        ),
        (
            "Annotations".to_string(),
            display::hashmap(ingress.metadata.annotations),
        ),
        ("Version".to_string(), ingress.metadata.resource_version),
    ];

    resource_info_view(&data)
        .attr("id", "ingress-info")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/ingresses/{name}/info/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

pub async fn rules_fragment(cache: &CacheStore, namespace: &str, name: &str) -> Element {
    let columns = rules_columns();
    let mut rows = find(cache, namespace, name)
        .await
        .spec
        .rules
        .into_iter()
        .map(|rule| {
            let paths = rule
                .http
                .paths
                .iter()
                .map(|p| p.path.clone())
                .collect::<Vec<_>>()
                .join("\n");
            let service_names = rule
                .http
                .paths
                .iter()
                .map(|p| p.backend.service.name.clone())
                .collect::<Vec<_>>()
                .join("\n");
            let service_ports = rule
                .http
                .paths
                .iter()
                .map(|p| p.backend.service.port.number.to_string())
                .collect::<Vec<_>>()
                .join("\n");

            vec![rule.host, paths, service_names, service_ports]
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a[1].cmp(&b[1]));

    let styles = vec![String::new(); columns.len()];
    let params = vec![String::new(); columns.len()];
    let rows = parse_table_rows(&columns, rows, &styles, &params);

    data_list_view(&columns, &rows)
        .attr("id", "ingress-rules")
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/ingresses/{name}/rules/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn rules_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Host", TableColumnType::String, 2),
        TableColumn::new("Path", TableColumnType::StringList, 2),
        TableColumn::new("Service Name", TableColumnType::String, 2),
        TableColumn::new("Service Port", TableColumnType::String, 2),
    ]
}
