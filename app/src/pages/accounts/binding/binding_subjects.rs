use leptos::prelude::*;
use leptos::task::spawn_local;

use api::accounts::bindings as bindings_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn RoleBindingSubjectsComponent(
    namespace_name: String,
    binding_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let binding_name = RwSignal::new(binding_name);
    let binding_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(namespace_name, binding_name, binding_data));
    clear_page_effect(interval_handle);
    view(binding_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    binding_name: RwSignal<String>,
    binding_data: RwSignal<Vec<Vec<String>>>,
) {
    if binding_name.is_disposed() || namespace_name.is_disposed() { return; }
    let namespace_name = namespace_name.get();
    let binding_name = binding_name.get();

    spawn_local(async move {
        let selected_value = if namespace_name == "All Namespaces" { None } else { Some(namespace_name) };
        let binding = bindings_api::get_rolebindings(selected_value).await
            .unwrap_or_default()
            .iter().find(|sc| sc.metadata.name == binding_name)
            .cloned()
            .unwrap_or_default();

        binding_data.set(binding.subjects
            .into_iter()
            .map(|r| vec![
                r.kind, r.name, r.namespace, r.api_group,
            ])
            .collect());
    });
}

fn view(
    resources: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 3),
        TableColumn::new("Name", TableColumnType::String, 3),
        TableColumn::new("Namespace", TableColumnType::String, 3),
        TableColumn::new("Api Group", TableColumnType::String, 3),
    ];
    let styles = vec![""; columns.len()];
    let params = vec![""; columns.len()];

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=resources.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
