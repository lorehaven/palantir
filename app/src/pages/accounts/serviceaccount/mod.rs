use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod serviceaccount_info;

#[component]
pub fn ServiceAccountPage() -> impl IntoView {
    let params = use_params_map();
    let namespace_name = params.with_untracked(|p| p.get("namespace"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let serviceaccount_name = params.with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let page_title = vec!["ServiceAccount".to_string(), namespace_name.clone(), serviceaccount_name.clone()];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="storageclass main-page">
                    <serviceaccount_info::ServiceAccountInfoComponent
                        serviceaccount_name=serviceaccount_name.clone()
                        namespace_name=namespace_name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
