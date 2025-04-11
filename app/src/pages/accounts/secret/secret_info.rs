use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;
use api::accounts::secrets as secrets_api;

#[component]
pub fn SecretInfoComponent(
    namespace_name: String,
    secret_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let secret_name = RwSignal::new(secret_name);
    let secret_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(namespace_name, secret_name, secret_data));
    clear_page_effect(interval_handle);

    resource_info_view(secret_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    secret_name: RwSignal<String>,
    secret_data: RwSignal<Vec<(String, String)>>,
) {
    if secret_name.is_disposed() || namespace_name.is_disposed() { return; }
    let namespace_name = namespace_name.get();
    let secret_name = secret_name.get();

    spawn_local(async move {
        let selected_value = if namespace_name == "All Namespaces" { None } else { Some(namespace_name) };
        let secret = secrets_api::get_secrets(selected_value).await
            .unwrap_or_default()
            .iter().find(|sc| sc.metadata.name == secret_name)
            .cloned()
            .unwrap_or_default();

        secret_data.set(vec![
            ("Name", secret.clone().metadata.name),
            ("Kind", "Secret".to_string()),
            ("Namespace", secret.clone().metadata.namespace),
            ("Created", format_timestamp(&secret.clone().metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Version", secret.metadata.resource_version),
            ("Type", secret.r#type),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
