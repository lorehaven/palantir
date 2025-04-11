use leptos::prelude::*;
use leptos::task::spawn_local;

use api::workloads::ingresses as ingresses_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn IngressesListComponent(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let ingresses = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(selected, prompt, ingresses));
    clear_page_effect(interval_handle);
    view(ingresses)
}

fn update_page(
    namespace_name: RwSignal<String>,
    ingress_name: RwSignal<String>,
    ingresses: RwSignal<Vec<Vec<String>>>,
) {
    if namespace_name.is_disposed() || ingress_name.is_disposed() { return; }
    let selected_value = namespace_name.get();
    let ingress_name = ingress_name.get();

    spawn_local(async move {
        let selected_value = if selected_value == "All Namespaces" { None } else { Some(selected_value) };
        let ingresses_data = ingresses_api::get_ingresses(selected_value).await.unwrap_or_default()
            .into_iter()
            .filter(|i| i.metadata.name.contains(&ingress_name))
            .collect::<Vec<_>>();

        let mut ingresses_vec = vec![];
        for ingress in ingresses_data {
            let hosts = ingress.clone()
                .spec.rules.into_iter()
                .map(|r| r.host)
                .collect::<Vec<_>>()
                .join("\n");
            let paths = ingress.clone()
                .spec.rules.into_iter()
                .map(|r| r.http.paths.into_iter()
                    .map(|p| p.path)
                    .collect::<Vec<_>>()
                    .join("\n"))
                .collect::<Vec<_>>()
                .join("\n");

            ingresses_vec.push(vec![
                "Ingress".to_string(),
                ingress.metadata.namespace,
                ingress.metadata.name,
                hosts,
                paths,
            ]);
        }
        ingresses.set(ingresses_vec);
    });
}

fn view(
    replicas: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 3),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Hosts", TableColumnType::StringList, 3),
        TableColumn::new("Paths", TableColumnType::StringList, 3),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/workloads/:1/ingresses/";

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=replicas.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
