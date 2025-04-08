use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::storage::claims as claims_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn ClaimsListComponent(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let claims = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(selected, prompt, claims));
    clear_page_effect(interval_handle);
    view(selected, claims)
}

fn update_page(
    namespace_name: RwSignal<String>,
    claim_name: RwSignal<String>,
    claims: RwSignal<Vec<Vec<String>>>,
) {
    spawn_local(async move {
        if namespace_name.is_disposed() || claim_name.is_disposed() { return; }

        let selected_value = if namespace_name.get_untracked() == "All Namespaces" { None } else { Some(namespace_name.get_untracked()) };
        let prompt_value = claim_name.get_untracked();
        let claims_data = claims_api::get_claims(selected_value).await.unwrap_or_default();

        claims.set(claims_data
            .into_iter()
            .filter(|n| n.metadata.name.to_lowercase().contains(&prompt_value.to_lowercase()))
            .map(|n| vec![
                "PersistentVolume".to_string(),
                n.clone().metadata.name,
                n.status.phase,
                n.spec.resources.requests.storage,
            ])
            .collect());
    });
}

fn view(
    namespace_name: RwSignal<String>,
    replicas: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 2),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Status", TableColumnType::String, 3),
        TableColumn::new("Capacity", TableColumnType::String, 3),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = format!("/storage/{}/claims/", namespace_name.get_untracked());

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
