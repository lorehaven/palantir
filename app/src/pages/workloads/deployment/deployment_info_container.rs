use api::workloads::deployments as deployments_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn DeploymentInfoContainerComponent(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || {
        update_page(namespace_name, resource_name, data);
    });
    clear_page_effect(interval_handle);

    view(data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    data: RwSignal<Vec<(&'static str, String)>>,
) {
    if namespace_name.is_disposed() || resource_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let resource_name = resource_name.get();

    spawn_local(async move {
        let deployment = deployments_api::get_deployments(None)
            .await
            .unwrap_or_default();
        let deployment = deployment
            .into_iter()
            .find(|n| n.metadata.namespace == namespace_name && n.metadata.name == resource_name)
            .unwrap_or_default();
        let container = deployment
            .spec
            .template
            .spec
            .containers
            .first()
            .cloned()
            .unwrap_or_default();

        data.set(vec![
            ("Container", container.name),
            ("Image", container.image),
            (
                "Env",
                container
                    .env
                    .into_iter()
                    .map(|e| format!("{}: {}", e.name, e.value))
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
            ("Cpu Request", container.resources.requests.cpu),
            ("Memory Request", container.resources.requests.memory),
            ("Cpu Limit", container.resources.limits.cpu),
            ("Memory Limit", container.resources.limits.memory),
            (
                "Ports",
                container
                    .ports
                    .into_iter()
                    .map(|p| {
                        let name = if p.name.is_empty() {
                            String::new()
                        } else {
                            format!("{} • ", p.name)
                        };
                        format!("{name}{} • {}", p.container_port, p.protocol)
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
        ]);
    });
}

fn view(data: RwSignal<Vec<(&'static str, String)>>) -> impl IntoView {
    view! {
        <div class="card-container dcc-1">
            <div class="card-list">
                {move || data.get().into_iter().map(|(name, value)| view! {
                    <div class="card-list-row">
                        <div class="card-list-row-title">{name}</div>
                        <div class="card-list-row-content">{value}</div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
