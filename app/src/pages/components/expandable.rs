use leptos::prelude::*;

#[slot]
pub struct ExpandableSlot {
    children: ChildrenFn,
}

#[component]
pub fn Expandable(
    expanded: bool,
    label: &'static str,
    expandable_slot: ExpandableSlot,
) -> impl IntoView {
    let value = RwSignal::new(expanded);

    view! {
        <div class="expandable-container">
            <div class="bar">
                <label>{label}</label>
                <div class="icon-button" on:click=move |_| *value.write() = !value.get()>
                    {move || if value.get() {
                        view! { <i class="fa-solid fa-circle-chevron-up" /> }
                    } else {
                        view! { <i class="fa-solid fa-circle-chevron-down" /> }
                    }}
                </div>
            </div>
            <div class="expandable" class:expanded=move || value.get()>
                { move || { (expandable_slot.children)().into_any() } }
            </div>
        </div>
    }
}
