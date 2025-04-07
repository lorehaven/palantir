use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

pub mod configmap_info;
pub mod configmap_data;

#[component]
pub fn WorkloadsConfigMapPage() -> impl IntoView {
    let params = use_params_map();
    let namespace_name = params.with_untracked(|p| p.get("namespace"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let configmap_name = params.with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>().join("-");
    let page_title = vec!["Workloads".to_string(), namespace_name.clone(), "ConfigMaps".to_string(), configmap_name.clone()];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-configmap main-page">
                    <configmap_info::ConfigMapInfoComponent
                        namespace_name=namespace_name.clone()
                        configmap_name=configmap_name.clone() />
                    <configmap_data::ConfigMapDataComponent
                        namespace_name=namespace_name.clone()
                        configmap_name=configmap_name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
