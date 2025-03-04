use leptos::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::events::EventsComponent;
use crate::components::stats::nodes::NodesStatComponent;
use crate::components::stats::pods::PodsStatComponent;

pub mod namespaces;
pub mod nodes;
pub mod node;

#[component]
pub fn ClusterPage() -> impl IntoView {
    view! {
        <Header text=vec!["Cluster"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster">
                    <NodesStatComponent />
                    <PodsStatComponent />
                    <EventsComponent />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
