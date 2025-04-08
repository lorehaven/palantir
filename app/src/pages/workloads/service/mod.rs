use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::events::EventsListComponent;
use crate::components::prelude::*;

pub mod service_info;

#[component]
pub fn WorkloadsServicePage() -> impl IntoView {
    let params = use_params_map();
    let namespace_name = params.with_untracked(|p| p.get("namespace"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let service_name = params.with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let page_title = vec!["Workloads".to_string(), namespace_name.clone(), "Service".to_string(), service_name.clone()];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-service main-page">
                    <service_info::ServiceInfoComponent
                        namespace_name=namespace_name.clone()
                        service_name=service_name.clone() />
                    <EventsListComponent
                        object_type="Service".to_string()
                        namespace_name=namespace_name.clone()
                        object_name=service_name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
