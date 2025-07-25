use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

pub mod configmap_data;
pub mod configmap_info;

#[component]
pub fn WorkloadsConfigMapPage() -> impl IntoView {
    let params = use_params_map();
    let namespace_name = params
        .with_untracked(|p| p.get("namespace"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec![
        "Workloads".to_string(),
        namespace_name.clone(),
        "ConfigMaps".to_string(),
        name.clone(),
    ];

    let resource_type = RwSignal::new("ConfigMap".to_string());
    let namespace_name = RwSignal::new(namespace_name);
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-configmap main-page">
                    <Actions
                        resource_type
                        namespace_name=namespace_name
                        resource_name=name
                        actions=&[ActionType::Edit, ActionType::Delete] />
                    <configmap_info::ConfigMapInfoComponent
                        namespace_name
                        resource_name=name />
                    <configmap_data::ConfigMapDataComponent
                        namespace_name
                        resource_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
