use api::resource as resource_api;
use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn PodLogsViewPage(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    selected_container: RwSignal<String>,
    follow_switch: RwSignal<bool>,
    previous_switch: RwSignal<bool>,
    prompt_value: RwSignal<String>,
) -> impl IntoView {
    let data = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);
    let log_ref = NodeRef::<leptos::html::Div>::new();

    let interval_handle = update_page_effect(5_000, move || {
        update_page(namespace_name, resource_name, selected_container, previous_switch, data, loading);
    });
    clear_page_effect(interval_handle);

    Effect::new(move |_| {
        data.get();
        if follow_switch.get() && !previous_switch.get() {
            if let Some(container) = log_ref.get() {
                container.set_scroll_top(container.scroll_height());
            }
        }
    });

    Effect::new(move |_| {
        previous_switch.get();
        data.set(vec![]);
        loading.set(true);
    });

    view(log_ref, data, prompt_value, loading)
}

fn update_page(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    container: RwSignal<String>,
    previous: RwSignal<bool>,
    data: RwSignal<Vec<String>>,
    loading: RwSignal<bool>,
) {
    if previous.get() && !data.get().is_empty() {
        return;
    }

    let namespace_name = namespace_name.get();
    let resource_name = resource_name.get();
    let container_name = container.get();
    let previous = previous.get();
    let tail_lines = if previous { -1 } else { 100 };

    spawn_local(async move {
        let logs = resource_api::logs(
            "Pod".to_string(),
            namespace_name,
            resource_name,
            container_name,
            previous,
            tail_lines,
        )
            .await
            .unwrap_or_default();


        data.update(|existing_logs| {
            if let Some(last) = existing_logs.last() {
                if let Some(pos) = logs.iter().position(|log| log == last) {
                    let new_entries = logs.iter().skip(pos + 1);
                    existing_logs.extend(new_entries.cloned());
                } else {
                    existing_logs.extend(logs);
                }
            } else {
                *existing_logs = logs;
            }
        });

        loading.set(false);
    });
}

fn view(
    log_ref: NodeRef<leptos::html::Div>,
    data: RwSignal<Vec<String>>,
    prompt: RwSignal<String>,
    loading: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="logs-view" node_ref=log_ref>
                    {move || if loading.get() {
                        view! { <div class="loader" /> }.into_any()
                    } else {
                        view! {
                            <pre>
                                {move || data.get()
                                    .into_iter()
                                    .filter(|entry| entry.to_lowercase().contains(&prompt.get().to_lowercase()))
                                    .map(|entry| view! {
                                    <div>{entry}</div>
                                }).collect::<Vec<_>>()}
                            </pre>
                        }.into_any()
                    }}
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
