use leptos::prelude::*;
use leptos::task::spawn_local;

use api::cluster::events as events_api;
use domain::utils::time::time_until_now;
use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn EventsComponent(
    prompt: RwSignal<String>,
) -> impl IntoView {
    let events = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(events, prompt));
    clear_page_effect(interval_handle);

    view(events)
}

fn update_page(
    events: RwSignal<Vec<Vec<String>>>,
    prompt: RwSignal<String>,
) {
    let prompt_value = prompt.get();
    spawn_local(async move {
        let events_list = events_api::get_events(None).await
            .unwrap_or_default()
            .into_iter()
            .filter(|e| e.involved_object.name.to_lowercase().contains(&prompt_value.to_lowercase()));
        events.set(events_list.map(|e| vec![
            e.involved_object.kind,
            e.involved_object.namespace,
            e.involved_object.name,
            time_until_now(&e.first_timestamp.unwrap_or_default()),
            e.reason,
            e.message,
        ]).collect());
    });
}

fn view(
    events: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 2),
        TableColumn::new("Name", TableColumnType::Link, 2),
        TableColumn::new("Time", TableColumnType::String, 1),
        TableColumn::new("Reason", TableColumnType::String, 2),
        TableColumn::new("Event", TableColumnType::String, 12),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/workloads/:1/";

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
