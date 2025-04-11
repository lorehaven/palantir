use leptos::prelude::*;

pub fn resource_info_view(
    data: RwSignal<Vec<(String, String)>>,
) -> impl IntoView {
    view! {
        <div class="card-container dcc-1">
            <div class="card-list">
                {move || data.get().into_iter().map(|(name, value)| view! {
                    <div class="card-list-row">
                        <div class="card-list-row-title">
                            {name}
                        </div>
                        <div class="card-list-row-content">
                            {value}
                        </div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

pub fn obscured_resource_info_view(
    data: RwSignal<Vec<(String, String)>>,
) -> impl IntoView {
    view! {
        <div class="card-container dcc-1">
            <div class="card-list">
                {move || data.get().into_iter().map(|(name, value)| {
                    let visible = RwSignal::new(false);

                    view! {
                        <div class="card-list-row-obscured">
                            <div class="card-list-row-title">
                                {name}
                            </div>
                            <div class="card-list-row-button">
                                <i
                                    class=move || if visible.get() { "fa-solid fa-lock-open" } else { "fa-solid fa-lock" }
                                    on:click=move |_| visible.update(|v| *v = !*v) />
                            </div>
                            <div class=move || if visible.get() { "card-list-row-content" } else { "card-list-row-content-obscured" }>
                                {value}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
