use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn StorageClaimsPage() -> impl IntoView {
    view! {
        <Header text=vec!["Storage", "Claims"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage-claims main-page">
                    Claims
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
