use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod volume_info;

#[component]
pub fn StorageVolumePage() -> impl IntoView {
    let params = use_params_map();
    let name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec![
        "Storage".to_string(),
        "Persistent Volumes".to_string(),
        name.clone(),
    ];

    let resource_type = RwSignal::new("PersistentVolume".to_string());
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage-volume main-page">
                    <Actions
                        resource_type
                        resource_name=name
                        actions=&[ActionType::Edit, ActionType::Delete] />
                    <volume_info::VolumeInfoComponent resource_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
