use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod storageclass_info;

#[component]
pub fn StorageClassPage() -> impl IntoView {
    let params = use_params_map();
    let storageclass_name = params.with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let page_title = vec!["StorageClass".to_string(), storageclass_name.clone()];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="storageclass main-page">
                    <storageclass_info::StorageClassInfoComponent storageclass_name=storageclass_name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
