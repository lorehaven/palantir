use leptos::prelude::*;
use leptos::task::spawn_local;

use api::cluster::events as events_api;
use crate::components::prelude::*;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use domain::utils::time::time_until_now;

#[component]
pub fn EventsListComponent(
    object_type: String,
    namespace_name: String,
    object_name: String,
) -> impl IntoView {
    let object_type = RwSignal::new(object_type);
    let namespace_name = RwSignal::new(namespace_name);
    let object_name = RwSignal::new(object_name);
    let events = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(
        object_type,
        namespace_name,
        object_name,
        events,
    ));
    clear_page_effect(interval_handle);

    view(events)
}

fn update_page(
    object_type: RwSignal<String>,
    namespace_name: RwSignal<String>,
    object_name: RwSignal<String>,
    events: RwSignal<Vec<Vec<String>>>,
) {
    if namespace_name.is_disposed() || object_name.is_disposed() { return; }
    let namespace_name = namespace_name.get();
    let object_type = object_type.get();
    let object_name = object_name.get();

    spawn_local(async move {
        let namespace_name = if namespace_name == "All Namespaces" { None } else { Some(namespace_name) };
        let mut events_data = events_api::get_events(namespace_name).await
            .unwrap_or_default()
            .into_iter()
            .filter(|e| e.involved_object.kind == object_type && e.involved_object.name == object_name)
            .collect::<Vec<_>>();
        events_data.sort_by(|a, b| a.metadata.creation_timestamp.cmp(&b.metadata.creation_timestamp));

        let mut events_vec = vec![];
        for event in events_data {
            events_vec.push(vec![
                time_until_now(&event.first_timestamp.unwrap_or_default()),
                event.reason,
                event.message,
            ]);
        }
        events.set(events_vec);
    });
}

fn view(
    events: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Time", TableColumnType::String, 1),
        TableColumn::new("Reason", TableColumnType::String, 1),
        TableColumn::new("Event", TableColumnType::String, 3),
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
                            values=events.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
