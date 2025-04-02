use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::services as services_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn ServicesListComponent(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let services = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(services, selected, prompt));
    clear_page_effect(interval_handle);
    view(services)
}

fn update_page(
    services: RwSignal<Vec<Vec<String>>>,
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) {
    let selected_value = selected.get();
    let prompt_value = prompt.get();
    spawn_local(async move {
        let selected_value = if selected_value == "All Namespaces" { None } else { Some(selected_value) };
        let services_data = services_api::get_services(selected_value).await.unwrap_or_default();

        services.set(services_data
            .into_iter()
            .filter(|s| s.metadata.name.to_lowercase().contains(&prompt_value.to_lowercase()))
            .map(|n| vec!["Service".to_string(), n.clone().metadata.namespace, n.clone().metadata.name])
            .collect());
    });
}

fn view(
    services: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 2),
        TableColumn::new("Name", TableColumnType::Link, 7),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/workloads/:1/services/";

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=services.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
