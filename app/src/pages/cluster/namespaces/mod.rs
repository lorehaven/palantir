use leptos::prelude::*;

use crate::components::prelude::*;

mod namespaces_list;

#[component]
pub fn ClusterNamespacesPage() -> impl IntoView {
    let resource_type = RwSignal::new("Namespaces".to_string());
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Cluster", "Namespaces"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-namespaces main-page">
                    <Actions
                        resource_type
                        prompt_value=resource_name
                        actions=&[ActionType::Prompt] />
                    <namespaces_list::NamespacesListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
