use leptos::prelude::*;
use leptos::task::spawn_local;

use api::storage::storageclasses as storage_api;
use crate::components::shared::info::resource_info_view;
use crate::pages::utils::shared::display;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::pages::utils::shared::time::format_timestamp;

#[component]
pub fn StorageClassInfoComponent(
    storageclass_name: String,
) -> impl IntoView {
    let storageclass_name = RwSignal::new(storageclass_name);
    let storageclass_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(storageclass_name, storageclass_data));
    clear_page_effect(interval_handle);

    resource_info_view(storageclass_data)
}

fn update_page(
    storageclass_name: RwSignal<String>,
    storageclass_data: RwSignal<Vec<(String, String)>>,
) {
    if storageclass_name.is_disposed() { return; }
    let storageclass_name = storageclass_name.get();

    spawn_local(async move {
        let storageclass = storage_api::get_storageclasses().await
            .unwrap_or_default()
            .iter().find(|sc| sc.metadata.name == storageclass_name)
            .cloned()
            .unwrap_or_default();

        storageclass_data.set(vec![
            ("Name", storageclass.clone().metadata.name),
            ("Kind", "StorageClass".to_string()),
            ("Created", format_timestamp(&storageclass.clone().metadata.creation_timestamp.unwrap_or_default(), None)),
            ("Labels", display::hashmap(storageclass.clone().metadata.labels)),
            ("Annotations", display::hashmap(storageclass.clone().metadata.annotations)),
            ("Version", storageclass.clone().metadata.resource_version),
            ("Provisioner", storageclass.clone().provisioner),
            ("Policy", storageclass.clone().reclaim_policy),
            ("Mode", storageclass.volume_binding_mode),
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    });
}
