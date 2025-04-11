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
    let ingress_name = params
        .with_untracked(|p| p.get("name"))
        .into_iter()
        .collect::<Vec<_>>()
        .join("-");
    let page_title = vec![
        "Workloads".to_string(),
        namespace_name.clone(),
        "Ingresses".to_string(),
        ingress_name.clone(),
    ];

    view! {
        <Header text=page_title />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads-ingress main-page">
                    <ingress_info::IngressInfoComponent
                        namespace_name=namespace_name.clone()
                        ingress_name=ingress_name.clone() />
                    <ingress_rules::IngressRulesComponent
                        namespace_name=namespace_name.clone()
                        ingress_name=ingress_name.clone() />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
