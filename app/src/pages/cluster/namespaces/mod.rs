use leptos::prelude::*;

use crate::components::prelude::*;

mod namespaces_list;

#[component]
pub fn ClusterNamespacesPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Cluster", "Namespaces"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-namespaces main-page">
                    <Filter
                        label="Namespaces"
                        resource_name
                        with_resource_name=true />
                    <namespaces_list::NamespacesListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
