use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::namespaces as namespaces_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::time_until_now;

#[component]
pub fn NamespacesListComponent() -> impl IntoView {
    let namespaces = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(namespaces));
    clear_page_effect(interval_handle);
    view(namespaces)
}

fn update_page(
    namespaces: RwSignal<Vec<Vec<String>>>,
) {
    spawn_local(async move {
        let namespaces_data = namespaces_api::get_namespaces().await.unwrap_or_default();

        let mut namespaces_vec = vec![];
        for namespace in namespaces_data {
            namespaces_vec.push(vec![
                "Namespace".to_string(),
                namespace.clone().metadata.name,
                time_until_now(&namespace.clone().metadata.creation_timestamp),
                namespace.clone().status.phase,
            ]);
        }
        namespaces.set(namespaces_vec);
    });
}

fn view(
    namespaces: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::Link, 2),
        TableColumn::new("Age", TableColumnType::String, 1),
        TableColumn::new("Status", TableColumnType::String, 1),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";

    view! {
        <Wrapper label="Namespaces">
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=namespaces.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
