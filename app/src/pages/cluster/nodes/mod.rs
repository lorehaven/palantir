use leptos::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::nodes::NodesStatComponent;

mod nodes_list;

#[component]
pub fn ClusterNodesPage() -> impl IntoView {
    let resource_type = RwSignal::new("Nodes".to_string());
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Cluster", "Nodes"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-nodes main-page">
                    <Actions
                        resource_type
                        prompt_value=resource_name
                        actions=&[ActionType::Prompt] />
                    <NodesStatComponent />
                    <nodes_list::NodesListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
