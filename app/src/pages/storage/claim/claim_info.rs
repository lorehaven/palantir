use api::storage::claims as claims_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn ClaimInfoComponent(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || {
        update_page(namespace_name, resource_name, data);
    });
    clear_page_effect(interval_handle);

    resource_info_view(data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    data: RwSignal<Vec<(String, String)>>,
) {
    if namespace_name.is_disposed() || resource_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let resource_name = resource_name.get();

    spawn_local(async move {
        let namespace_name = if namespace_name == "All Namespaces" {
            None
        } else {
            Some(namespace_name)
        };
        let claim = claims_api::get_claims(namespace_name)
            .await
            .unwrap_or_default()
            .iter()
            .find(|sc| sc.metadata.name == resource_name)
            .cloned()
            .unwrap_or_default();

        data.set(
            vec![
                ("Name", claim.clone().metadata.name),
                ("Kind", "PersistentVolumeClaim".to_string()),
                ("Namespace", claim.clone().metadata.namespace),
                (
                    "Created",
                    format_timestamp(
                        &claim
                            .clone()
                            .metadata
                            .creation_timestamp
                            .unwrap_or_default(),
                        None,
                    ),
                ),
                ("Labels", display::hashmap(claim.clone().metadata.labels)),
                (
                    "Annotations",
                    display::hashmap(claim.clone().metadata.annotations),
                ),
                ("Version", claim.clone().metadata.resource_version),
                ("Status", claim.clone().status.phase),
                ("Class", String::new()),
                ("Volume", claim.clone().spec.volume_name),
                ("Modes", claim.spec.access_modes.join("\n")),
                ("Capacity", claim.spec.resources.requests.storage),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
