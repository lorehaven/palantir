use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod secret_data;
mod secret_info;

#[component]
pub fn AccountsSecretPage() -> impl IntoView {
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
        "Accounts".to_string(),
        namespace_name.clone(),
        "Secrets".to_string(),
        name.clone(),
    ];

    let resource_type = RwSignal::new("Secret".to_string());
    let namespace_name = RwSignal::new(namespace_name);
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-secret main-page">
                    <Actions
                        resource_type
                        namespace_name=namespace_name
                        resource_name=name
                        actions=&[ActionType::Edit, ActionType::Delete] />
                    <secret_info::SecretInfoComponent namespace_name resource_name=name />
                    <secret_data::SecretDataComponent namespace_name resource_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
