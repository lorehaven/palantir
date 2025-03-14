use leptos::prelude::*;

use crate::components::prelude::*;

pub mod claims;
pub mod volumes;

#[component]
pub fn StoragePage() -> impl IntoView {
    view! {
        <Header text=vec!["Storage"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage main-page">
                    Storage
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
