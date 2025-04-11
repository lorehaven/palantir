use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use domain::utils::time::time_until_now;
use api::cluster::nodes as nodes_api;

#[component]
pub fn NodeConditionsComponent(
    node_name: String,
) -> impl IntoView {
    let node_name = RwSignal::new(node_name);
    let conditions = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(node_name, conditions));
    clear_page_effect(interval_handle);

    view(conditions)
}

fn update_page(
    node_name: RwSignal<String>,
    conditions: RwSignal<Vec<Vec<String>>>,
) {
    if node_name.is_disposed() { return; }
    let node_name = node_name.get();

    spawn_local(async move {
        let node = nodes_api::get_node_by_name(node_name).await
            .unwrap_or_default();
        let mut conditions_vec = vec![];
        for condition in node.status.conditions {
            conditions_vec.push(vec![
                condition.r#type,
                condition.status,
                time_until_now(&condition.last_transition_time),
                condition.reason,
                condition.message,
            ]);
        }
        conditions.set(conditions_vec);
    });
}

fn view(
    conditions: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Condition", TableColumnType::String, 1),
        TableColumn::new("Status", TableColumnType::String, 1),
        TableColumn::new("Transition", TableColumnType::String, 1),
        TableColumn::new("Reason", TableColumnType::String, 2),
        TableColumn::new("Message", TableColumnType::String, 2),
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
                            values=conditions.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
