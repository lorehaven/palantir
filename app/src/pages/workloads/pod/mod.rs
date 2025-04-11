use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::events::EventsListComponent;
use crate::components::prelude::*;

pub mod pod_info;
pub mod pod_info_container;
pub mod pod_stats;

#[component]
pub fn WorkloadsPodPage() -> impl IntoView {
    let params = use_params_map();
    let namespace_name = params
        .with_untracked(|p| p.get("namespace"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let pod_name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec![
        "Workloads".to_string(),
        namespace_name.clone(),
        "Pod".to_string(),
        pod_name.clone(),
    ];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-pod main-page">
                    <pod_stats::PodStatsComponent
                        namespace_name=namespace_name.clone()
                        pod_name=pod_name.clone() />
                    <pod_info::PodInfoComponent
                        namespace_name=namespace_name.clone()
                        pod_name=pod_name.clone() />
                    <pod_info_container::PodInfoContainerComponent
                        namespace_name=namespace_name.clone()
                        pod_name=pod_name.clone() />
                    <EventsListComponent
                        object_type="Pod".to_string()
                        namespace_name=namespace_name.clone()
                        object_name=pod_name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
