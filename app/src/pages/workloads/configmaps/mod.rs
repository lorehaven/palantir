use leptos::prelude::*;

use crate::components::prelude::*;

pub mod configs_list;

#[component]
pub fn WorkloadsConfigMapsPage() -> impl IntoView {
    let resource_type = RwSignal::new("ConfigMaps".to_string());
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "ConfigMaps"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-config main-page">
                    <Actions
                        resource_type
                        selected_namespace=namespace_name
                        prompt_value=resource_name
                        actions=&[ActionType::NamespacesFilter, ActionType::Prompt] />
                    <configs_list::ConfigsListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
