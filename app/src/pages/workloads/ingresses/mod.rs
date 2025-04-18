use leptos::prelude::*;

use crate::components::prelude::*;

pub mod ingresses_list;

#[component]
pub fn WorkloadsIngressesPage() -> impl IntoView {
    let resource_type = RwSignal::new("Ingresses".to_string());
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Ingresses"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-ingresses main-page">
                    <Actions
                        resource_type
                        selected_namespace=namespace_name
                        prompt_value=resource_name
                        actions=&[ActionType::NamespacesFilter, ActionType::Prompt] />
                    <ingresses_list::IngressesListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
