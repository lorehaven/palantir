use leptos::prelude::*;
use crate::components::cluster::nodes::NodesListComponent;
use crate::components::prelude::*;
use crate::components::stats::nodes::NodesStatComponent;

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
