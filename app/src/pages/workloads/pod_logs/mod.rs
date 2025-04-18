use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

pub mod logs_view;

#[component]
pub fn WorkloadsPodLogsPage() -> impl IntoView {
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

    let resource_type = RwSignal::new("Pod".to_string());
    let namespace_name = RwSignal::new(namespace_name);
    let name = RwSignal::new(name);
    let selected_container = RwSignal::new(String::new());
    let follow_switch = RwSignal::new(true);
    let previous_switch = RwSignal::new(false);
    let prompt_value = RwSignal::new(String::new());

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-pod-logs main-page">
                    <Actions
                        resource_type
                        namespace_name=namespace_name
                        resource_name=name
                        selected_container
                        follow_switch
                        previous_switch
                        prompt_value
                        actions=&[
                            ActionType::ContainersFilter,
                            ActionType::Follow,
                            ActionType::Previous,
                            ActionType::Prompt,
                            ActionType::Save,
                        ] />
                    <logs_view::PodLogsViewPage
                        namespace_name=namespace_name
                        resource_name=name
                        selected_container
                        follow_switch
                        previous_switch
                        prompt_value />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
