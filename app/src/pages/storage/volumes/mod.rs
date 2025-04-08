use leptos::prelude::*;

use crate::components::prelude::*;

mod volumes_list;

#[component]
pub fn StorageVolumesPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Storage", "Volumes"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage-volumes main-page">
                    <Filter
                        label="Persistent Volumes"
                        prompt
                        with_prompt=true />
                    <volumes_list::VolumesListComponent prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
