use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn WorkloadsServicesPage() -> impl IntoView {
    view! {
        <Header text=vec!["Workloads", "Services"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-services main-page">
                    Services
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
