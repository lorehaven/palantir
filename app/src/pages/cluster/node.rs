use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::components::prelude::PageContent;
use crate::components::prelude::*;

#[component]
pub fn ClusterNodePage(

) -> impl IntoView {
    let params = use_params_map();
    let node_name = params.read().get("name").unwrap_or("".to_string());
    let page_title = vec!["Cluster".to_string(), "Nodes".to_string(), node_name];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-node">

                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
