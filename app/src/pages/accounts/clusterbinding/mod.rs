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

    let resource_type = RwSignal::new("ClusterRoleBinding".to_string());
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-cluster-role-binding main-page">
                    <Actions
                        resource_type
                        resource_name=name
                        actions=&[ActionType::Edit, ActionType::Delete] />
                    <cluster_binding_info::ClusterRoleBindingInfoComponent resource_name=name />
                    <cluster_binding_subjects::ClusterRoleBindingSubjectsComponent resource_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
