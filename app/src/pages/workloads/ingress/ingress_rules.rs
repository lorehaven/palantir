use leptos::prelude::*;
use leptos::task::spawn_local;

use api::workloads::ingresses as ingresses_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn IngressRulesComponent(
    namespace_name: String,
    ingress_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let ingress_name = RwSignal::new(ingress_name);
    let rules = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(namespace_name, ingress_name, rules));
    clear_page_effect(interval_handle);
    view(rules)
}

fn update_page(
    namespace_name: RwSignal<String>,
    ingress_name: RwSignal<String>,
    rules: RwSignal<Vec<Vec<String>>>,
) {
    if namespace_name.is_disposed() || ingress_name.is_disposed() { return; }
    let selected_value = namespace_name.get();
    let ingress_name = ingress_name.get();

    spawn_local(async move {
        let selected_value = if selected_value == "All Namespaces" { None } else { Some(selected_value) };
        let ingress = ingresses_api::get_ingresses(selected_value).await.unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == ingress_name)
            .unwrap_or_default();

        let mut rules_vec = vec![];
        for rule in ingress.spec.rules {
            let paths = rule.clone()
                .http.paths.into_iter()
                .map(|p| p.path)
                .collect::<Vec<_>>()
                .join("\n");
            let service_names = rule.clone()
                .http.paths.into_iter()
                .map(|p| p.backend.service.name)
                .collect::<Vec<_>>()
                .join("\n");
            let service_ports = rule.clone()
                .http.paths.into_iter()
                .map(|p| p.backend.service.port.number.to_string())
                .collect::<Vec<_>>()
                .join("\n");

            rules_vec.push(vec![
                rule.clone().host,
                paths,
                service_names,
                service_ports,
            ]);
        }
        rules.set(rules_vec);
    });
}

fn view(
    rules: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Host", TableColumnType::String, 2),
        TableColumn::new("Path", TableColumnType::StringList, 2),
        TableColumn::new("Service Name", TableColumnType::String, 2),
        TableColumn::new("Service Port", TableColumnType::String, 2),
    ];
    let styles = vec![""; columns.len()];
    let params = vec![""; columns.len()];

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=rules.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
