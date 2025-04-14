use leptos::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::pods::PodsStatComponent;

pub mod pods_list;

#[component]
pub fn WorkloadsPodsPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Pods"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-pods main-page">
                    <Filter
                        label="Pods"
                        namespace_name
                        resource_name
                        with_namespace=true
                        with_resource_name=true />
                    <PodsStatComponent namespace_name />
                    <pods_list::PodsListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
