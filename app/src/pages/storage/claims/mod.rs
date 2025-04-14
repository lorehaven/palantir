use leptos::prelude::*;

use crate::components::prelude::*;

mod claims_list;

#[component]
pub fn StorageClaimsPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Storage", "Claims"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage-claims main-page">
                    <Filter
                        label="Persistent Volume Claims"
                        namespace_name
                        resource_name
                        with_namespace=true
                        with_resource_name=true />
                    <claims_list::ClaimsListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
