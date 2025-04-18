use leptos::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::events::EventsComponent;
use crate::components::stats::nodes::NodesStatComponent;
use crate::components::stats::pods::PodsStatComponent;

pub mod namespace;
pub mod namespaces;
pub mod node;
pub mod nodes;

#[component]
pub fn ClusterPage() -> impl IntoView {
    let resource_type = RwSignal::new("Cluster".to_string());
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Cluster"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster main-page">
                    <Actions
                        resource_type
                        prompt_value=resource_name
                        actions=&[ActionType::Prompt] />
                    <NodesStatComponent />
                    <PodsStatComponent />
                    <EventsComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
