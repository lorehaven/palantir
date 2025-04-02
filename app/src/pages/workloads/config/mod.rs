use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn WorkloadsConfigPage() -> impl IntoView {
    view! {
        <Header text=vec!["Workloads", "Config"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-config main-page">
                    Config
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
