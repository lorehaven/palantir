use api::accounts::serviceaccounts as serviceaccounts_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn ServiceAccountInfoComponent(
    namespace_name: String,
    serviceaccount_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let serviceaccount_name = RwSignal::new(serviceaccount_name);
    let serviceaccount_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || {
        update_page(namespace_name, serviceaccount_name, serviceaccount_data)
    });
    clear_page_effect(interval_handle);

    resource_info_view(serviceaccount_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    serviceaccount_name: RwSignal<String>,
    serviceaccount_data: RwSignal<Vec<(String, String)>>,
) {
    if namespace_name.is_disposed() || serviceaccount_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let serviceaccount_name = serviceaccount_name.get();

    spawn_local(async move {
        let selected_value = if namespace_name == "All Namespaces" {
            None
        } else {
            Some(namespace_name)
        };
        let sa = serviceaccounts_api::get_serviceaccounts(selected_value)
            .await
            .unwrap_or_default()
            .iter()
            .find(|sc| sc.metadata.name == serviceaccount_name)
            .cloned()
            .unwrap_or_default();

        serviceaccount_data.set(
            vec![
                ("Name", sa.clone().metadata.name),
                ("Kind", "ServiceAccount".to_string()),
                ("Namespace", sa.clone().metadata.namespace),
                (
                    "Created",
                    format_timestamp(
                        &sa.clone().metadata.creation_timestamp.unwrap_or_default(),
                        None,
                    ),
                ),
                ("Labels", display::hashmap(sa.clone().metadata.labels)),
                (
                    "Annotations",
                    display::hashmap(sa.clone().metadata.annotations),
                ),
                ("Version", sa.metadata.resource_version),
                ("Secrets", String::new()),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
