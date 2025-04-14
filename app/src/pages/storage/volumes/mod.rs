use leptos::prelude::*;

use crate::components::prelude::*;

mod volumes_list;

#[component]
pub fn StorageVolumesPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Storage", "Volumes"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage-volumes main-page">
                    <Filter
                        label="Persistent Volumes"
                        resource_name
                        with_resource_name=true />
                    <volumes_list::VolumesListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
