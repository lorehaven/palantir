use leptos::prelude::*;

use crate::components::prelude::*;

pub mod services_list;

#[component]
pub fn WorkloadsServicesPage() -> impl IntoView {
    let resource_type = RwSignal::new("Services".to_string());
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Services"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-services main-page">
                    <Actions
                        resource_type
                        selected_namespace=namespace_name
                        prompt_value=resource_name
                        actions=&[ActionType::NamespacesFilter, ActionType::Prompt] />
                    <services_list::ServicesListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
