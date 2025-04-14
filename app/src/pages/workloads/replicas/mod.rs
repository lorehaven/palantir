use leptos::prelude::*;

use crate::components::prelude::*;

pub mod replicasets_list;

#[component]
pub fn WorkloadsReplicaSetsPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Replicas"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-replicas main-page">
                    <Filter
                        label="Replica Sets"
                        namespace_name
                        resource_name
                        with_namespace=true
                        with_resource_name=true />
                    <replicasets_list::ReplicaSetsListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
