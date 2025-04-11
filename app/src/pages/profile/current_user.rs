use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::text::decode_jwt_token;
use api::utils::get_api_token_wasm;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::Value;

#[component]
pub fn CurrentUserComponent() -> impl IntoView {
    let token = RwSignal::new(String::new());

    let interval_handle = update_page_effect(3_600_000, move || update_page(token));
    clear_page_effect(interval_handle);

    view(token)
}

fn update_page(
    token: RwSignal<String>,
) {
    spawn_local(async move {
        let encoded = get_api_token_wasm().await.unwrap_or_default();
        let decoded = serde_json::from_str::<Value>(&decode_jwt_token(&encoded)).unwrap();
        let decoded = serde_json::to_string_pretty(&decoded).unwrap();
        token.set(decoded);
    });
}

fn view(
    token: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div class="card-container dcc-1">
            <div class="card-list">
                <div class="card-list-row">
                    <div class="card-list-row-title">Token</div>
                    <div class="card-list-row-content">
                        <pre>{ move || token.get() }</pre>
                    </div>
                </div>
            </div>
        </div>
    }
}
