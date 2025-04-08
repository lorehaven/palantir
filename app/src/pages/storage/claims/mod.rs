use leptos::prelude::*;

use crate::components::prelude::*;

mod claims_list;

#[component]
pub fn StorageClaimsPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());
    let selected = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Storage", "Claims"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="storage-claims main-page">
                    <Filter
                        label="Persistent Volume Claims"
                        selected
                        prompt
                        with_namespace=true
                        with_prompt=true />
                    <claims_list::ClaimsListComponent selected prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
