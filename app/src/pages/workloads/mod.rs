use leptos::prelude::*;

use crate::components::prelude::*;

pub mod config;
pub mod ingresses;
pub mod pods;
pub mod replicas;
pub mod services;

#[component]
pub fn WorkloadsPage() -> impl IntoView {
    view! {
        <Header text=vec!["Workloads"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads main-page">
                    Workloads
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
