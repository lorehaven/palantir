use leptos::prelude::*;

#[component]
pub fn CardString(label: &'static str, label_add: &'static str, value: String) -> impl IntoView {
    view! {
        <div class="card-string">
            <div>
                <div class="label">{label}</div>
                <div class="label-add">{label_add}</div>
            </div>
            <div class="card-string-content"> { value } </div>
        </div>
    }
}
