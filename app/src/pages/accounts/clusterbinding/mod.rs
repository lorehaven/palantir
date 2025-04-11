use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod cluster_binding_info;
mod cluster_binding_subjects;

#[component]
pub fn AccountsClusterRoleBindingPage() -> impl IntoView {
    let params = use_params_map();
    let name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec![
        "Accounts".to_string(),
        "Cluster Role Bindings".to_string(),
        name.clone(),
    ];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-cluster-role-binding main-page">
                    <cluster_binding_info::ClusterRoleBindingInfoComponent cluster_binding_name=name.clone() />
                    <cluster_binding_subjects::ClusterRoleBindingSubjectsComponent cluster_binding_name=name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
