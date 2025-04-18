use leptos::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::pods::PodsStatComponent;

pub mod pods_list;

#[component]
pub fn WorkloadsPodsPage() -> impl IntoView {
    let resource_type = RwSignal::new("Pods".to_string());
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Pods"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-pods main-page">
                    <Actions
                        resource_type
                        selected_namespace=namespace_name
                        prompt_value=resource_name
                        actions=&[ActionType::NamespacesFilter, ActionType::Prompt] />
                    <PodsStatComponent namespace_name />
                    <pods_list::PodsListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
