use leptos::prelude::*;

#[component]
pub fn FilterLabel(label: &'static str) -> impl IntoView {
    view! {
        <div class="filter-label">{ label }</div>
    }
}
