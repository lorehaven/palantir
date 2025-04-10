use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::storage::claims as claims_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::time_until_now;

#[component]
pub fn ClaimsListComponent(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let claims = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(selected, prompt, claims));
    clear_page_effect(interval_handle);
    view(claims)
}

fn update_page(
    namespace_name: RwSignal<String>,
    claim_name: RwSignal<String>,
    claims: RwSignal<Vec<Vec<String>>>,
) {
    if namespace_name.is_disposed() || claim_name.is_disposed() { return; }
    let selected_value = namespace_name.get();
    let prompt_value = claim_name.get();

    spawn_local(async move {
        let selected_value = if selected_value == "All Namespaces" { None } else { Some(selected_value) };
        let claims_data = claims_api::get_claims(selected_value).await.unwrap_or_default();

        claims.set(claims_data
            .into_iter()
            .filter(|n| n.metadata.name.to_lowercase().contains(&prompt_value.to_lowercase()))
            .map(|n| vec![
                "PersistentVolume".to_string(),
                n.clone().metadata.namespace,
                n.clone().metadata.name,
                time_until_now(&n.metadata.creation_timestamp.unwrap_or_default()),
                n.status.phase,
                n.spec.storage_class_name,
                n.spec.volume_name,
                n.spec.resources.requests.storage,
            ])
            .collect());
    });
}

fn view(
    replicas: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 2),
        TableColumn::new("Namespace", TableColumnType::Link, 3),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Age", TableColumnType::String, 3),
        TableColumn::new("Status", TableColumnType::String, 3),
        TableColumn::new("Class Name", TableColumnType::String, 3),
        TableColumn::new("Volume", TableColumnType::String, 3),
        TableColumn::new("Capacity", TableColumnType::String, 3),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/storage/:1/claims/";

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
