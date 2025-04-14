use leptos::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::nodes::NodesStatComponent;

mod nodes_list;

#[component]
pub fn ClusterNodesPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Cluster", "Nodes"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-nodes main-page">
                    <Filter
                        label="Nodes"
                        resource_name
                        with_resource_name=true />
                    <NodesStatComponent />
                    <nodes_list::NodesListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
