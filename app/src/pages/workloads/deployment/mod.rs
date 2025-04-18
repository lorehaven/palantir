use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::events::EventsListComponent;
use crate::components::prelude::*;

pub mod deployment_info;
pub mod deployment_info_container;
pub mod deployment_pods;
pub mod deployment_replicasets;
pub mod deployment_stats;

#[component]
pub fn WorkloadsDeploymentPage() -> impl IntoView {
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
        "Workloads".to_string(),
        namespace_name.clone(),
        "Deployment".to_string(),
        name.clone(),
    ];

    let resource_type = RwSignal::new("Deployment".to_string());
    let namespace_name = RwSignal::new(namespace_name);
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-deployment main-page">
                    <Actions
                        resource_type
                        namespace_name=namespace_name
                        resource_name=name
                        actions=&[ActionType::Edit, ActionType::Delete] />
                    <deployment_stats::DeploymentStatsComponent
                        namespace_name
                        resource_name=name />
                    <deployment_info::DeploymentInfoComponent
                        namespace_name
                        resource_name=name />
                    <deployment_info_container::DeploymentInfoContainerComponent
                        namespace_name
                        resource_name=name />
                    <deployment_replicasets::DeploymentReplicaSetsComponent
                        namespace_name
                        resource_name=name />
                    <deployment_pods::DeploymentPodsComponent
                        namespace_name
                        resource_name=name />
                    <EventsListComponent
                        object_type="Deployment".to_string()
                        namespace_name
                        object_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
