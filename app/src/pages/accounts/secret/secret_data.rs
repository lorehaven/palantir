use leptos::prelude::*;
use leptos::task::spawn_local;

use api::accounts::secrets as secrets_api;
use crate::components::shared::info::obscured_resource_info_view;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn SecretDataComponent(
    namespace_name: String,
    secret_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let secret_name = RwSignal::new(secret_name);
    let secret_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(
        namespace_name,
        secret_name,
        secret_data,
    ));
    clear_page_effect(interval_handle);

    obscured_resource_info_view(secret_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    secret_name: RwSignal<String>,
    secret_data: RwSignal<Vec<(String, String)>>,
) {
    if namespace_name.is_disposed() || secret_name.is_disposed() { return; }
    let namespace_name = namespace_name.get();
    let secret_name = secret_name.get();

    spawn_local(async move {
        let selected_value = if namespace_name == "All Namespaces" { None } else { Some(namespace_name) };
        let secret = secrets_api::get_secrets(selected_value).await.unwrap_or_default()
            .into_iter()
            .find(|n| n.metadata.name == secret_name)
            .unwrap_or_default();

        secret_data.set(secret.data.into_iter().map(|(k, v)| (k, v.replace('\n', " "))).collect());
    });
}
