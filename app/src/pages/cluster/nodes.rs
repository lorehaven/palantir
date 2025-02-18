use leptos::prelude::*;
use crate::pages::components::cluster::nodes::NodesListComponent;
use crate::pages::components::prelude::*;
use crate::pages::components::stats::nodes::NodesStatComponent;

#[component]
pub fn ClusterNodesPage() -> impl IntoView {
    view! {
        <Header text=" > Cluster > Nodes" />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-nodes">
                    <NodesStatComponent />
                    <NodesListComponent />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
