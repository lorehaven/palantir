use leptos::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::nodes::NodesStatComponent;

mod nodes_list;

#[component]
pub fn ClusterNodesPage() -> impl IntoView {
    view! {
        <Header text=vec!["Cluster", "Nodes"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-nodes">
                    <NodesStatComponent />
                    <nodes_list::NodesListComponent />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
