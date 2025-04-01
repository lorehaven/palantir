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
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let namespaces = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || update_page(namespaces));
    clear_page_effect(interval_handle);

    view(label, selected, namespaces, prompt)
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
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="filter">
                    <label::FilterLabel label />
                    <namespace::NamespaceFilter namespaces selected />
                    <spacer::FilterSpacer />
                    <prompt::PromptFilter prompt />
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
