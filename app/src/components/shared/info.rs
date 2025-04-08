use leptos::prelude::*;

pub fn resource_info_view(
    data: RwSignal<Vec<(String, String)>>,
) -> impl IntoView {
    view! {
        <div class="card-container dcc-1">
            <div class="card-list">
                {move || data.get().into_iter().map(|(name, value)| view! {
                    <div class="card-list-row">
                        <div class="card-list-row-title">{name}</div>
                        <div class="card-list-row-content">{value}</div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
