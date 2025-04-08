use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod volume_info;

#[component]
pub fn StorageVolumePage() -> impl IntoView {
    let params = use_params_map();
    let volume_name = params.with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let page_title = vec!["Storage".to_string(), "Persistent Volumes".to_string(), volume_name.clone()];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage-volume main-page">
                    <volume_info::VolumeInfoComponent volume_name=volume_name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
