use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use api::workloads::replicasets as replicasets_api;

#[component]
pub fn ReplicaSetInfoContainerComponent(
    namespace_name: String,
    replicaset_name: String,
) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let replicaset_name = RwSignal::new(replicaset_name);
    let replicaset_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(
        namespace_name,
        replicaset_name,
        replicaset_data,
    ));
    clear_page_effect(interval_handle);

    view(replicaset_data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    replicaset_name: RwSignal<String>,
    replicaset_data: RwSignal<Vec<(&'static str, String)>>,
) {
    if namespace_name.is_disposed() || replicaset_name.is_disposed() { return; }
    let selected_value = namespace_name.get();
    let replicaset_name = replicaset_name.get();

    spawn_local(async move {
        let replicaset = replicasets_api::get_replicasets(None).await
            .unwrap_or_default();
        let replicaset = replicaset.into_iter()
            .find(|n| n.metadata.namespace == selected_value && n.metadata.name == replicaset_name)
            .unwrap_or_default();
        let container = replicaset.spec.template.spec.containers.first().cloned().unwrap_or_default();

        let mut items = vec![];
        items.push(("Container", container.name));
        items.push(("Image", container.image));
        items.push(("Env", container.env.into_iter()
            .map(|e| format!("{}: {}", e.name, e.value))
            .collect::<Vec<String>>()
            .join("\n")));
        items.push(("Cpu Request", container.resources.requests.cpu));
        items.push(("Memory Request", container.resources.requests.memory));
        items.push(("Cpu Limit", container.resources.limits.cpu));
        items.push(("Memory Limit", container.resources.limits.memory));
        items.push(("Ports", container.ports.into_iter()
            .map(|p| {
                let name = if p.name.is_empty() { String::new() } else { format!("{} • ", p.name) };
                format!("{name}{} • {}", p.container_port, p.protocol)
            })
            .collect::<Vec<String>>()
            .join("\n")));
        replicaset_data.set(items);
    });
}

fn view(
    replicaset_data: RwSignal<Vec<(&'static str, String)>>,
) -> impl IntoView {
    view! {
        <div class="card-container dcc-1">
            <div class="card-list">
                {move || replicaset_data.get().into_iter().map(|(name, value)| view! {
                    <div class="card-list-row">
                        <div class="card-list-row-title">{name}</div>
                        <div class="card-list-row-content">{value}</div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
