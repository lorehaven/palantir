use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn WorkloadsPodsPage() -> impl IntoView {
    view! {
        <Header text=vec!["Workloads", "Pods"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-pods main-page">
                    Pods
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
