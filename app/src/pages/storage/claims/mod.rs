use leptos::prelude::*;

use crate::components::prelude::*;

mod claims_list;

#[component]
pub fn StorageClaimsPage() -> impl IntoView {
    let resource_type = RwSignal::new("PersistentVolumeClaims".to_string());
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Storage", "Claims"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage-claims main-page">
                    <Actions
                        resource_type
                        selected_namespace=namespace_name
                        prompt_value=resource_name
                        actions=&[ActionType::NamespacesFilter, ActionType::Prompt] />
                    <claims_list::ClaimsListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
