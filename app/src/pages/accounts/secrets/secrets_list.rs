use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use api::accounts::secrets as secrets_api;
use domain::utils::time::time_until_now;

#[component]
pub fn SecretsListComponent(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let secrets = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(selected, prompt, secrets));
    clear_page_effect(interval_handle);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 2),
        TableColumn::new("Name", TableColumnType::Link, 4),
        TableColumn::new("Age", TableColumnType::String, 1),
        TableColumn::new("Type", TableColumnType::String, 2),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/accounts/:1/secrets/";
    data_list_view(columns, secrets, styles, params)
}

fn update_page(
    namespace_name: RwSignal<String>,
    secret_name: RwSignal<String>,
    secrets: RwSignal<Vec<Vec<String>>>,
) {
    if namespace_name.is_disposed() || secret_name.is_disposed() { return; }
    let selected_value = namespace_name.get();
    let secret_name = secret_name.get();

    spawn_local(async move {
        let selected_value = if selected_value == "All Namespaces" { None } else { Some(selected_value) };
        let mut secrets_data = secrets_api::get_secrets(selected_value).await.unwrap_or_default();
        secrets_data.sort_by(|a, b| a.metadata.name.cmp(&b.metadata.name));

        secrets.set(secrets_data
            .into_iter()
            .filter(|s| s.metadata.name.to_lowercase().contains(&secret_name.to_lowercase()))
            .map(|s| vec![
                "Secret".to_string(),
                s.clone().metadata.namespace,
                s.clone().metadata.name,
                time_until_now(&s.clone().metadata.creation_timestamp.unwrap_or_default()),
                s.r#type,
            ])
            .collect());
    });
}
