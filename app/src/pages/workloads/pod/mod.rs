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
    let name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec![
        "Workloads".to_string(),
        namespace_name.clone(),
        "Pod".to_string(),
        name.clone(),
    ];

    let namespace_name = RwSignal::new(namespace_name);
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-pod main-page">
                    <pod_stats::PodStatsComponent
                        namespace_name
                        resource_name=name />
                    <pod_info::PodInfoComponent
                        namespace_name
                        resource_name=name />
                    <pod_info_container::PodInfoContainerComponent
                        namespace_name
                        resource_name=name />
                    <EventsListComponent
                        object_type="Pod".to_string()
                        namespace_name
                        object_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
