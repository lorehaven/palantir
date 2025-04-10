use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::storage::claims as claims_api;
use crate::components::shared::info::resource_info_view;
use crate::pages::utils::shared::display;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

#[component]
pub fn ClaimInfoComponent(
    namespace_name: String,
    claim_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let claim_name = RwSignal::new(claim_name);
    let claim_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(
        namespace_name,
        claim_name,
        claim_data,
    ));
    clear_page_effect(interval_handle);

    resource_info_view(claim_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    claim_name: RwSignal<String>,
    claim_data: RwSignal<Vec<(String, String)>>,
) {
    if namespace_name.is_disposed() || claim_name.is_disposed() { return; }
    let namespace_name = namespace_name.get();
    let claim_name = claim_name.get();

    spawn_local(async move {
        let selected_value = if namespace_name == "All Namespaces" { None } else { Some(namespace_name) };
        let claim = claims_api::get_claims(selected_value).await
            .unwrap_or_default()
            .iter().find(|sc| sc.metadata.name == claim_name)
            .cloned()
            .unwrap_or_default();

        claim_data.set(vec![
            ("Name", claim.clone().metadata.name),
            ("Kind", "PersistentVolumeClaim".to_string()),
            ("Namespace", claim.clone().metadata.namespace),
            ("Created", format_timestamp(&claim.clone().metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Labels", display::hashmap(claim.clone().metadata.labels)),
            ("Annotations", display::hashmap(claim.clone().metadata.annotations)),
            ("Version", claim.clone().metadata.resource_version),
            ("Status", claim.clone().status.phase),
            ("Class", String::new()),
            ("Volume", claim.clone().spec.volume_name),
            ("Modes", claim.spec.access_modes.join("\n")),
            ("Capacity", claim.spec.resources.requests.storage),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
