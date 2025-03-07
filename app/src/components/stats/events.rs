use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::events as events_api;
use crate::components::prelude::*;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::time_until_now;

#[component]
pub fn EventsComponent() -> impl IntoView {
    let events = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(events));
    clear_page_effect(interval_handle);

    view(events)
}

fn update_page(
    events: RwSignal<Vec<Vec<String>>>,
) {
    spawn_local(async move {
        let events_list = events_api::get_events().await.unwrap_or_default();
        events.set(events_list.into_iter().map(|e|
            vec![e.involved_object.kind, e.involved_object.name, time_until_now(&e.first_timestamp), e.reason, e.message]
        ).collect());
    });
}

fn view(
    events: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::String, 4),
        TableColumn::new("Time", TableColumnType::String, 1),
        TableColumn::new("Reason", TableColumnType::String, 2),
        TableColumn::new("Event", TableColumnType::String, 12),
    ];
    let styles = vec![""; columns.len()];
    let params = vec![""; columns.len()];

    view! {
        <Expandable label="Events" expanded=true>
            <ExpandableSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=events.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </ExpandableSlot>
        </Expandable>
    }
}
