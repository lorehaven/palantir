use leptos::prelude::*;

use crate::pages::components::prelude::*;

#[component]
pub fn ClusterNodesPage() -> impl IntoView {
    view! {
        <Header text=" > Cluster > Nodes" />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-nodes">
                    Nodes
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
