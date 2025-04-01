use leptos::prelude::*;
use leptos::task::spawn_local;

pub mod label;
pub mod spacer;

pub mod namespace;
pub mod prompt;

use crate::api::namespaces as namespaces_api;
use crate::components::prelude::*;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn Filter(
    label: &'static str,
    #[prop(default = RwSignal::new("".to_string()))]
    selected: RwSignal<String>,
    #[prop(default = RwSignal::new("".to_string()))]
    prompt: RwSignal<String>,
    #[prop(default = false)]
    with_namespace: bool,
    #[prop(default = false)]
    with_prompt: bool,
) -> impl IntoView {
    let namespaces = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(namespaces));
    clear_page_effect(interval_handle);

    view(label, selected, namespaces, prompt, with_namespace, with_prompt)
}

fn update_page(namespaces: RwSignal<Vec<String>>) {
    spawn_local(async move {
        let mut namespaces_names = namespaces_api::get_namespaces()
            .await.unwrap_or_default()
            .into_iter()
            .map(|n| n.metadata.name)
            .collect::<Vec<_>>();
        namespaces_names.insert(0, "All Namespaces".to_string());
        namespaces.set(namespaces_names);
    });
}

fn view(
    label: &'static str,
    selected: RwSignal<String>,
    namespaces: RwSignal<Vec<String>>,
    prompt: RwSignal<String>,
    with_namespace: bool,
    with_prompt: bool,
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="filter">
                    <label::FilterLabel label />
                    <Show when=move || with_namespace>
                        <namespace::NamespaceFilter namespaces selected />
                    </Show>
                    <spacer::FilterSpacer />
                    <Show when=move || with_prompt>
                        <prompt::PromptFilter prompt />
                    </Show>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
