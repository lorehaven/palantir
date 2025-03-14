use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn WorkloadsReplicasPage() -> impl IntoView {
    view! {
        <Header text=vec!["Workloads", "Replicas"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-replicas main-page">
                    Replicas
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
