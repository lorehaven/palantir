use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod serviceaccount_info;

#[component]
pub fn ServiceAccountPage() -> impl IntoView {
    let params = use_params_map();
    let namespace_name = params
        .with_untracked(|p| p.get("namespace"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec![
        "ServiceAccounts".to_string(),
        namespace_name.clone(),
        name.clone(),
    ];

    let resource_type = RwSignal::new("ServiceAccount".to_string());
    let namespace_name = RwSignal::new(namespace_name);
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="storageclass main-page">
                    <Actions
                        resource_type
                        namespace_name=namespace_name
                        resource_name=name
                        actions=&[ActionType::Edit, ActionType::Delete] />
                    <serviceaccount_info::ServiceAccountInfoComponent
                        resource_name=name
                        namespace_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
