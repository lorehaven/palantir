use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::accounts::roles as roles_api;
use crate::components::prelude::{TableColumn, TableColumnType, TableComponent, Wrapper, WrapperSlot};
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn RolesListComponent(
    prompt: RwSignal<String>,
) -> impl IntoView {
    let roles = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(roles, prompt));
    clear_page_effect(interval_handle);
    view(roles)
}

fn update_page(
    roles: RwSignal<Vec<Vec<String>>>,
    prompt: RwSignal<String>,
) {
    if prompt.is_disposed() { return; }
    let prompt_value = prompt.get();

    spawn_local(async move {
        let roles_list = roles_api::get_all_roles().await;
        let mut list = roles_list.into_iter()
            .filter(|r| r.get_name().to_lowercase().contains(&prompt_value.to_lowercase()))
            .map(|r| r.to_model())
            .map(|r| vec![
                r.r#type, r.namespace, r.name, r.age,
            ]).collect::<Vec<_>>();
        list.sort_by(|a, b| a[1].cmp(&b[1]));
        roles.set(list);
    });
}

fn view(
    roles: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 2),
        TableColumn::new("Namespace", TableColumnType::String, 2),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Age", TableColumnType::String, 1),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/accounts/:1/:0s/";

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=roles.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
