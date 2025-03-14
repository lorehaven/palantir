use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn StorageVolumesPage() -> impl IntoView {
    view! {
        <Header text=vec!["Storage", "Volumes"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage-volumes main-page">
                    Volumes
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
