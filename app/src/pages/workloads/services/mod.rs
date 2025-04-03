use leptos::prelude::*;

use crate::components::prelude::*;

pub mod services_list;

#[component]
pub fn WorkloadsServicesPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());
    let selected = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Services"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-services main-page">
                    <Filter
                        label="Services"
                        selected
                        prompt
                        with_namespace=true
                        with_prompt=true />
                    <services_list::ServicesListComponent selected prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
