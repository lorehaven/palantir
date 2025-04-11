use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod binding_info;
mod binding_subjects;

#[component]
pub fn AccountsRoleBindingPage() -> impl IntoView {
    let params = use_params_map();
    let namespace_name = params.with_untracked(|p| p.get("namespace"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let name = params.with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let page_title = vec!["Accounts".to_string(), namespace_name.clone(), "Role Bindings".to_string(), name.clone()];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-role-binding main-page">
                    <binding_info::RoleBindingInfoComponent namespace_name=namespace_name.clone() binding_name=name.clone() />
                    <binding_subjects::RoleBindingSubjectsComponent namespace_name=namespace_name.clone() binding_name=name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
