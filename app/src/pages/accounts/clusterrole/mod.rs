use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

mod cluster_role_info;
mod cluster_role_rules;

#[component]
pub fn AccountsClusterRolePage() -> impl IntoView {
    let params = use_params_map();
    let name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec![
        "Accounts".to_string(),
        "Cluster Roles".to_string(),
        name.clone(),
    ];

    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-cluster-role main-page">
                    <cluster_role_info::ClusterRoleInfoComponent resource_name=name />
                    <cluster_role_rules::ClusterRoleRulesComponent resource_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
