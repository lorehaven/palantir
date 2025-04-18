use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod binding_info;
mod binding_subjects;

#[component]
pub fn AccountsRoleBindingPage() -> impl IntoView {
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
        "Role Bindings".to_string(),
        name.clone(),
    ];

    let resource_type = RwSignal::new("RoleBinding".to_string());
    let namespace_name = RwSignal::new(namespace_name);
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-role-binding main-page">
                    <Actions
                        resource_type
                        namespace_name=namespace_name
                        resource_name=name
                        actions=&[ActionType::Edit, ActionType::Delete] />
                    <binding_info::RoleBindingInfoComponent namespace_name resource_name=name />
                    <binding_subjects::RoleBindingSubjectsComponent namespace_name resource_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
