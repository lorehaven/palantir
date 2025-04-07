use leptos::prelude::*;

use crate::components::prelude::*;

pub mod configs_list;

#[component]
pub fn WorkloadsConfigMapsPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());
    let selected = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads", "Configs"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-config main-page">
                    <Filter
                        label="Ingresses"
                        selected
                        prompt
                        with_namespace=true
                        with_prompt=true />
                    <configs_list::ConfigsListComponent selected prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
