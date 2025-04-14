use leptos::prelude::*;

use crate::components::prelude::*;

pub mod services_list;

#[component]
pub fn WorkloadsServicesPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Services"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-services main-page">
                    <Filter
                        label="Services"
                        namespace_name
                        resource_name
                        with_namespace=true
                        with_resource_name=true />
                    <services_list::ServicesListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
