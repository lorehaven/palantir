use api::storage::volumes as volumes_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::shared::data::resource_info_view;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn VolumeInfoComponent(volume_name: String) -> impl IntoView {
    let volume_name = RwSignal::new(volume_name);
    let volume_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(volume_name, volume_data));
    clear_page_effect(interval_handle);

    resource_info_view(volume_data)
}

fn update_page(volume_name: RwSignal<String>, volume_data: RwSignal<Vec<(String, String)>>) {
    if volume_name.is_disposed() {
        return;
    }
    let volume_name = volume_name.get();

    spawn_local(async move {
        let volume = volumes_api::get_volumes()
            .await
            .unwrap_or_default()
            .iter()
            .find(|sc| sc.metadata.name == volume_name)
            .cloned()
            .unwrap_or_default();

        volume_data.set(
            vec![
                ("Name", volume.clone().metadata.name),
                ("Kind", "PersistentVolume".to_string()),
                (
                    "Created",
                    format_timestamp(
                        &volume
                            .clone()
                            .metadata
                            .creation_timestamp
                            .unwrap_or_default(),
                        None,
                    ),
                ),
                ("Labels", display::hashmap(volume.clone().metadata.labels)),
                (
                    "Annotations",
                    display::hashmap(volume.clone().metadata.annotations),
                ),
                ("Version", volume.clone().status.phase),
                ("Status", volume.clone().status.phase),
                ("Class", String::new()),
                (
                    "Claim",
                    format!(
                        "{}/{}",
                        volume.clone().spec.claim_ref.namespace,
                        volume.spec.claim_ref.name
                    ),
                ),
                ("Access Modes", volume.spec.access_mode.join("\n")),
                ("Capacity", volume.spec.capacity.storage),
                (
                    "Reclaim Policy",
                    volume.spec.persistent_volume_reclaim_policy,
                ),
                ("Local Path", String::new()),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
    });
}
