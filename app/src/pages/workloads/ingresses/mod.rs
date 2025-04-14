use leptos::prelude::*;

use crate::components::prelude::*;

pub mod ingresses_list;

#[component]
pub fn WorkloadsIngressesPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Ingresses"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-ingresses main-page">
                    <Filter
                        label="Ingresses"
                        namespace_name
                        resource_name
                        with_namespace=true
                        with_resource_name=true />
                    <ingresses_list::IngressesListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
