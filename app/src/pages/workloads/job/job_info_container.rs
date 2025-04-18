use api::workloads::pods as pods_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn JobInfoContainerComponent(
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
        let pod = pods_api::get_pods(Some(namespace_name.clone()), None)
            .await
            .unwrap_or_default()
            .into_iter()
            .find(|p| p.metadata.labels.get("job-name").cloned().unwrap_or_default() == resource_name)
            .unwrap_or_default();
        let container = pod
            .spec
            .containers
            .first()
            .cloned()
            .unwrap_or_default();

        data.set(vec![
            ("Container", container.name),
            ("Image", container.image),
            ("Args", container.args.join(" ")),
            (
                "Env",
                container
                    .env
                    .into_iter()
                    .map(|e| format!("{}: {}", e.name, e.value))
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
