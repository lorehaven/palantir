use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::configmaps as configs_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn ConfigsListComponent(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let configs = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(selected, prompt, configs));
    clear_page_effect(interval_handle);
    view(selected, configs)
}

fn update_page(
    namespace_name: RwSignal<String>,
    config_name: RwSignal<String>,
    configs: RwSignal<Vec<Vec<String>>>,
) {
    spawn_local(async move {
        if namespace_name.is_disposed() || config_name.is_disposed() { return; }

        let selected_value = if namespace_name.get_untracked() == "All Namespaces" { None } else { Some(namespace_name.get_untracked()) };
        let configs_data = configs_api::get_configmaps(selected_value).await.unwrap_or_default()
            .into_iter()
            .filter(|i| i.metadata.name.contains(&config_name.get_untracked()))
            .collect::<Vec<_>>();

        let mut configs_vec = vec![];
        for config in configs_data {
            configs_vec.push(vec![
                "ConfigMap".to_string(),
                config.metadata.name,
            ]);
        }
        configs.set(configs_vec);
    });
}

fn view(
    namespace_name: RwSignal<String>,
    replicas: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::Link, 9),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = format!("/workloads/{}/configmaps/", namespace_name.get_untracked());

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
