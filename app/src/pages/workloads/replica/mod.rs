use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::events::EventsListComponent;
use crate::components::prelude::*;

pub mod replicaset_info;
pub mod replicaset_info_container;
pub mod replicaset_list;
pub mod replicaset_stats;

#[component]
pub fn WorkloadsReplicaSetPage() -> impl IntoView {
    let params = use_params_map();
    let namespace_name = params.with_untracked(|p| p.get("namespace"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let replicaset_name = params.with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let page_title = vec!["Workloads".to_string(), namespace_name.clone(), "Replica Set".to_string(), replicaset_name.clone()];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-replicaset main-page">
                    <replicaset_stats::ReplicaSetsStatsComponent
                        namespace_name=namespace_name.clone()
                        replicaset_name=replicaset_name.clone() />
                    <replicaset_info::ReplicaSetInfoComponent
                        namespace_name=namespace_name.clone()
                        replicaset_name=replicaset_name.clone() />
                    <replicaset_info_container::ReplicaSetInfoContainerComponent
                        namespace_name=namespace_name.clone()
                        replicaset_name=replicaset_name.clone() />
                    <replicaset_list::ReplicaSetListComponent
                        namespace_name=namespace_name.clone()
                        replicaset_name=replicaset_name.clone() />
                    <EventsListComponent
                        object_type="ReplicaSet".to_string()
                        namespace_name=namespace_name.clone()
                        object_name=replicaset_name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
