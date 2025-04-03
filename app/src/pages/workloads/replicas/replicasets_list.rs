use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::replicasets as replicasets_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn ReplicaSetsListComponent(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let replicasets = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(replicasets, selected, prompt));
    clear_page_effect(interval_handle);
    view(replicasets)
}

fn update_page(
    replicasets: RwSignal<Vec<Vec<String>>>,
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) {
    let selected_value = selected.get();
    let prompt_value = prompt.get();
    spawn_local(async move {
        let selected_value = if selected_value == "All Namespaces" { None } else { Some(selected_value) };
        let replicasets_data = replicasets_api::get_replicasets(selected_value).await.unwrap_or_default();

        replicasets.set(replicasets_data
            .into_iter()
            .filter(|s| s.metadata.name.to_lowercase().contains(&prompt_value.to_lowercase()))
            .map(|r| vec![
                "ReplicaSet".to_string(),
                r.clone().metadata.namespace,
                r.clone().metadata.name,
                r.clone().metadata.generation.to_string(),
                format!("{}/{}", r.clone().status.available_replicas, r.clone().status.replicas),
            ])
            .collect());
    });
}

fn view(
    replicasets: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 2),
        TableColumn::new("Name", TableColumnType::Link, 4),
        TableColumn::new("Generations", TableColumnType::String, 1),
        TableColumn::new("Replicas", TableColumnType::String, 2),
        // TableColumn::new("Active", TableColumnType::Switch, 2),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/workloads/:1/replicasets/";

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=replicasets.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
