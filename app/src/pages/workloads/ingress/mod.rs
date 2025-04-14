use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::prelude::*;

pub mod ingress_info;
pub mod ingress_rules;

#[component]
pub fn WorkloadsIngressPage() -> impl IntoView {
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
        "Ingresses".to_string(),
        name.clone(),
    ];

    let namespace_name = RwSignal::new(namespace_name);
    let name = RwSignal::new(name);

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-ingress main-page">
                    <ingress_info::IngressInfoComponent
                        namespace_name
                        resource_name=name />
                    <ingress_rules::IngressRulesComponent
                        namespace_name
                        resource_name=name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
