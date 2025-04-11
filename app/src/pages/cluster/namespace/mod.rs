use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;
use crate::components::stats::pods::PodsStatComponent;

mod namespace_events;
mod namespace_info;
mod namespace_pods;

#[component]
pub fn ClusterNamespacePage() -> impl IntoView {
    let params = use_params_map();
    let namespace_name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec![
        "Cluster".to_string(),
        "Namespaces".to_string(),
        namespace_name.clone(),
    ];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-namespace main-page">
                    <PodsStatComponent namespace_name=Some(namespace_name.clone()) expandable=false />
                    <namespace_info::NamespaceInfoComponent namespace_name=namespace_name.clone() />
                    <namespace_pods::NamespacePodsComponent namespace_name=namespace_name.clone() />
                    <namespace_events::NamespaceEventsComponent namespace_name=namespace_name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
