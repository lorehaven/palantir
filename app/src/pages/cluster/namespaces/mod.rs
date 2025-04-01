use leptos::prelude::*;

use crate::components::prelude::*;

mod namespaces_list;

#[component]
pub fn ClusterNamespacesPage() -> impl IntoView {
    let prompt = RwSignal::new("".to_string());

    view! {
        <Header text=vec!["Cluster", "Namespaces"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-namespaces main-page">
                    <Filter
                        label="Namespaces"
                        prompt
                        with_prompt=true />
                    <namespaces_list::NamespacesListComponent prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
