use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn ClusterNamespacesPage() -> impl IntoView {
    view! {
        <Header text=" > Cluster > Namespaces" />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-namespaces">
                    Namespaces
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
