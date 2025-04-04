use leptos::prelude::*;

use crate::components::prelude::*;
use crate::components::stats::pods::PodsStatComponent;

pub mod pods_list;

#[component]
pub fn WorkloadsPodsPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());
    let selected = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Pods"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-pods main-page">
                    <Filter
                        label="Pods"
                        selected
                        prompt
                        with_namespace=true
                        with_prompt=true />
                    <PodsStatComponent namespace_name=if selected.get() == "All Namespaces" { None } else { Some(selected.get()) } />
                    <pods_list::PodsListComponent selected prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
