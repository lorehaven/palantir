use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::pages::components::prelude::*;
use crate::pages::utils::shared::effects::update_page_effect;
use crate::pages::utils::shared::time::time_until_now;

#[component]
pub fn EventsComponent() -> impl IntoView {
    let events = RwSignal::new(vec![]);

    update_page_effect(5_000, move || update_page(events));
    view(events)
}

fn update_page(
    events: RwSignal<Vec<Vec<String>>>,
) {
    spawn_local(async move {
        let events_list = crate::api::events::get_events().await.unwrap_or_default();
        events.set(events_list.into_iter().map(|e|
            vec![e.involved_object.kind, e.involved_object.name, time_until_now(&e.first_timestamp), e.reason, e.message]
        ).collect());
    });
}

fn view(
    events: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    view! {
        <Expandable label="Events" expanded=true>
            <ExpandableSlot slot>
                <div class="card-container dcc-1">
                    <DashboardCardList
                        labels=&["Type", "Name", "Time", "Reason", "Event"]
                        widths=&["5%", "20%", "5%", "10%", "60%"]
                        rows=events.get() />
                </div>
            </ExpandableSlot>
        </Expandable>
    }
}
