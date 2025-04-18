use api::storage::storageclasses as storage_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

pub mod storageclass;

pub mod claim;
pub mod claims;
pub mod volume;
pub mod volumes;

#[component]
pub fn StorageClassesPage() -> impl IntoView {
    let resource_type = RwSignal::new("StorageClasses".to_string());
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Storage"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage main-page">
                    <Actions
                        resource_type
                        prompt_value=resource_name
                        actions=&[ActionType::Prompt] />
                    <StorageClassesListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}

#[component]
fn StorageClassesListComponent(resource_name: RwSignal<String>) -> impl IntoView {
    let table_rows = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 2),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Reclaim Policy", TableColumnType::String, 3),
        TableColumn::new("Mode", TableColumnType::String, 3),
    ];
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/storageclasses/".to_string();

    let columns_update = columns.clone();
    let interval_handle = update_page_effect(10_000, move || {
        update_page_list(
            columns_update.clone(),
            styles.clone(),
            params.clone(),
            table_rows,
            resource_name,
            loading,
        );
    });
    clear_page_effect(interval_handle);
    data_list_view(columns, table_rows, loading)
}

fn update_page_list(
    columns: Vec<TableColumn>,
    styles: Vec<String>,
    params: Vec<String>,
    table_rows: RwSignal<Vec<TableRow>>,
    resource_name: RwSignal<String>,
    loading: RwSignal<bool>,
) {
    if resource_name.is_disposed() {
        return;
    }
    let resource_name = resource_name.get();

    spawn_local(async move {
        let list = update_page_list_async(
            columns.clone(),
            styles.clone(),
            params.clone(),
            resource_name.clone(),
        )
        .await
        .unwrap_or_default();
        table_rows.set(list);
        loading.set(false);
    });
}

#[server]
async fn update_page_list_async(
    columns: Vec<TableColumn>,
    styles: Vec<String>,
    params: Vec<String>,
    resource_name: String,
) -> Result<Vec<TableRow>, ServerFnError> {
    let mut list = storage_api::get_storageclasses()
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|n| {
            n.metadata
                .name
                .to_lowercase()
                .contains(&resource_name.to_lowercase())
        })
        .map(|n| {
            vec![
                "StorageClass".to_string(),
                n.clone().metadata.name,
                n.clone().reclaim_policy,
                n.volume_binding_mode,
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    Ok(parse_table_rows(columns, list, styles, params))
}
