use leptos::prelude::*;
use leptos::task::spawn_local;

use api::storage::volumes as volumes_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn VolumesListComponent(
    prompt: RwSignal<String>,
) -> impl IntoView {
    let volumes = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(volumes, prompt));
    clear_page_effect(interval_handle);
    view(volumes)
}

fn update_page(
    volumes: RwSignal<Vec<Vec<String>>>,
    prompt: RwSignal<String>,
) {
    if prompt.is_disposed() { return; }
    let prompt_value = prompt.get();

    spawn_local(async move {
        let volumes_data = volumes_api::get_volumes().await.unwrap_or_default();

        volumes.set(volumes_data
            .into_iter()
            .filter(|n| n.metadata.name.to_lowercase().contains(&prompt_value.to_lowercase()))
            .map(|n| vec![
                "PersistentVolume".to_string(),
                n.clone().metadata.name,
                n.status.phase,
                n.spec.capacity.storage,
            ])
            .collect());
    });
}

fn view(
    volumes: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 2),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Status", TableColumnType::String, 3),
        TableColumn::new("Capacity", TableColumnType::String, 3),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/storage/volumes/";

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=volumes.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
