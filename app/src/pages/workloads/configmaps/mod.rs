use leptos::prelude::*;

use crate::components::prelude::*;

pub mod configs_list;

#[component]
pub fn WorkloadsConfigMapsPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Configs"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-config main-page">
                    <Filter
                        label="Ingresses"
                        namespace_name
                        resource_name
                        with_namespace=true
                        with_resource_name=true />
                    <configs_list::ConfigsListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
