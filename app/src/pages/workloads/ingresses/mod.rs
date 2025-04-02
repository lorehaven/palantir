use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn WorkloadsIngressesPage() -> impl IntoView {
    view! {
        <Header text=vec!["Workloads", "Ingresses"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-ingresses main-page">
                    Ingresses
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
