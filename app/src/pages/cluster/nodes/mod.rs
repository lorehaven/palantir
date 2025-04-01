use leptos::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::nodes::NodesStatComponent;

mod nodes_list;

#[component]
pub fn ClusterNodesPage() -> impl IntoView {
    let prompt = RwSignal::new("".to_string());

    view! {
        <Header text=vec!["Cluster", "Nodes"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-nodes main-page">
                    <Filter
                        label="Nodes"
                        prompt
                        with_prompt=true />
                    <NodesStatComponent />
                    <nodes_list::NodesListComponent prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
