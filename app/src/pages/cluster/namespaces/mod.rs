use leptos::prelude::*;

use crate::components::prelude::*;

mod namespaces_list;

#[component]
pub fn ClusterNamespacesPage() -> impl IntoView {
    view! {
        <Header text=vec!["Cluster", "Namespaces"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-namespaces main-page">
                    <namespaces_list::NamespacesListComponent />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
