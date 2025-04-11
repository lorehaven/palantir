use leptos::prelude::*;
use leptos::task::spawn_local;

use api::storage::storageclasses as storage_api;
use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

pub mod storageclass;

pub mod claim;
pub mod claims;
pub mod volume;
pub mod volumes;

#[component]
pub fn StorageClassesPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Storage"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage main-page">
                    <Filter
                        label="Storage Classes"
                        prompt
                        with_prompt=true />
                    <StorageClassesListComponent prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}

#[component]
fn StorageClassesListComponent(
    prompt: RwSignal<String>,
) -> impl IntoView {
    let classes = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(3_600_000, move || update_page_list(prompt, classes));
    clear_page_effect(interval_handle);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 2),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Reclaim Policy", TableColumnType::String, 3),
        TableColumn::new("Mode", TableColumnType::String, 3),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/storageclasses/";
    data_list_view(columns, classes, styles, params)
}

fn update_page_list(
    prompt: RwSignal<String>,
    classes: RwSignal<Vec<Vec<String>>>,
) {
    if prompt.is_disposed() || classes.is_disposed() { return; }
    let prompt_value = prompt.get();

    spawn_local(async move {
        let classes_data = storage_api::get_storageclasses().await.unwrap_or_default();

        classes.set(classes_data
            .into_iter()
            .filter(|n| n.metadata.name.to_lowercase().contains(&prompt_value.to_lowercase()))
            .map(|n| vec![
                "StorageClass".to_string(),
                n.clone().metadata.name,
                n.clone().reclaim_policy,
                n.volume_binding_mode,
            ])
            .collect());
    });
}
