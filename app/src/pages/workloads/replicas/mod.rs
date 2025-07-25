use leptos::prelude::*;

use crate::components::prelude::*;

pub mod replicasets_list;

#[component]
pub fn WorkloadsReplicaSetsPage() -> impl IntoView {
    let resource_type = RwSignal::new("Replicas".to_string());
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Replicas"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-replicas main-page">
                    <Actions
                        resource_type
                        selected_namespace=namespace_name
                        prompt_value=resource_name
                        actions=&[ActionType::NamespacesFilter, ActionType::Prompt] />
                    <replicasets_list::ReplicaSetsListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
