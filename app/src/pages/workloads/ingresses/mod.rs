use leptos::prelude::*;

use crate::components::prelude::*;

pub mod ingresses_list;

#[component]
pub fn WorkloadsIngressesPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());
    let selected = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Ingresses"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-ingresses main-page">
                    <Filter
                        label="Ingresses"
                        selected
                        prompt
                        with_namespace=true
                        with_prompt=true />
                    <ingresses_list::IngressesListComponent selected prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
