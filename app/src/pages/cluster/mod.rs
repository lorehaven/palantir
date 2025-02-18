use leptos::prelude::*;

use crate::pages::components::prelude::*;
use crate::pages::components::stats::events::EventsComponent;
use crate::pages::components::stats::nodes::NodesStatComponent;
use crate::pages::components::stats::pods::PodsStatComponent;

pub mod namespaces;
pub mod nodes;

#[component]
pub fn ClusterPage() -> impl IntoView {
    view! {
        <Header text=" > Cluster" />
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
