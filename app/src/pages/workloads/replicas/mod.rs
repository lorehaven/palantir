use leptos::prelude::*;

use crate::components::prelude::*;

pub mod replicasets_list;

#[component]
pub fn WorkloadsReplicaSetsPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());
    let selected = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Replicas"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-replicas main-page">
                    <Filter
                        label="Replica Sets"
                        selected
                        prompt
                        with_namespace=true
                        with_prompt=true />
                    <replicasets_list::ReplicaSetsListComponent selected prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
