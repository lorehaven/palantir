use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;
use crate::components::stats::nodes::NodesStatComponent;
use crate::components::stats::pods::PodsStatComponent;
use crate::pages::cluster::node::node_conditions::NodeConditionsComponent;
use crate::pages::cluster::node::node_info::NodeInfoComponent;
use crate::pages::cluster::node::node_pods::NodePodsComponent;

mod node_conditions;
mod node_info;
mod node_pods;

#[component]
pub fn ClusterNodePage() -> impl IntoView {
    let params = use_params_map();
    let name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec!["Cluster".to_string(), "Nodes".to_string(), name.clone()];

    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="cluster-node main-page">
                    <NodesStatComponent node_name=name expandable=false />
                    <PodsStatComponent node_name=name expandable=false />
                    <NodeInfoComponent resource_name=name />
                    <NodeConditionsComponent resource_name=name />
                    <NodePodsComponent resource_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
