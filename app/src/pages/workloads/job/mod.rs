use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::events::EventsListComponent;
use crate::components::prelude::*;

pub mod job_info;
pub mod job_info_container;
pub mod job_pods;
pub mod job_stats;

#[component]
pub fn WorkloadsJobPage() -> impl IntoView {
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
        "Job".to_string(),
        name.clone(),
    ];

    let resource_type = RwSignal::new("Job".to_string());
    let namespace_name = RwSignal::new(namespace_name);
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-job main-page">
                    <Actions
                        resource_type
                        namespace_name=namespace_name
                        resource_name=name
                        actions=&[ActionType::Edit, ActionType::Delete] />
                    <job_stats::JobStatsComponent
                        namespace_name
                        resource_name=name />
                    <job_info::JobInfoComponent
                        namespace_name
                        resource_name=name />
                    <job_info_container::JobInfoContainerComponent
                        namespace_name
                        resource_name=name />
                    <job_pods::JobPodsComponent
                        namespace_name
                        resource_name=name />
                    <EventsListComponent
                        object_type="Job".to_string()
                        namespace_name
                        object_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
