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
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Cluster"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster main-page">
                    <Filter
                        label="Cluster overview"
                        resource_name
                        with_resource_name=true />
                    <NodesStatComponent />
                    <PodsStatComponent />
                    <EventsComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
