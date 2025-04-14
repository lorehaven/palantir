use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod role_info;
mod role_rules;

#[component]
pub fn AccountsRolePage() -> impl IntoView {
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
        "Roles".to_string(),
        name.clone(),
    ];

    let namespace_name = RwSignal::new(namespace_name);
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-role main-page">
                    <role_info::RoleInfoComponent namespace_name resource_name=name />
                    <role_rules::RoleRulesComponent namespace_name resource_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
