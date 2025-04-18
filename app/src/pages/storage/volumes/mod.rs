use leptos::prelude::*;

use crate::components::prelude::*;

mod volumes_list;

#[component]
pub fn StorageVolumesPage() -> impl IntoView {
    let resource_type = RwSignal::new("PersistentVolumes".to_string());
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Storage", "Volumes"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage-volumes main-page">
                    <Actions
                        resource_type
                        prompt_value=resource_name
                        actions=&[ActionType::Prompt] />
                    <volumes_list::VolumesListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
