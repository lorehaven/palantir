use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod storageclass_info;

#[component]
pub fn StorageClassPage() -> impl IntoView {
    let params = use_params_map();
    let name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec!["StorageClasses".to_string(), name.clone()];

    let resource_type = RwSignal::new("StorageClass".to_string());
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="storageclass main-page">
                    <Actions
                        resource_type
                        resource_name=name
                        actions=&[ActionType::Edit, ActionType::Delete] />
                    <storageclass_info::StorageClassInfoComponent
                        resource_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
