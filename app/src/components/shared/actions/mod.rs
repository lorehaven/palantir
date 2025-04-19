use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::prelude::*;

pub mod containers;
pub mod delete;
pub mod edit;
pub mod follow;
pub mod logs;
pub mod namespaces;
pub mod previous;
pub mod prompt;
pub mod save;
pub mod scale;

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq)]
pub enum ActionType {
    ContainersFilter,
    Delete,
    Save,
    Edit,
    Follow,
    Logs,
    NamespacesFilter,
    Previous,
    Prompt,
    Scale,
}

#[component]
pub fn Actions(
    resource_type: RwSignal<String>,
    #[prop(default = RwSignal::new(String::new()))] namespace_name: RwSignal<String>,
    #[prop(default = RwSignal::new(String::new()))] resource_name: RwSignal<String>,
    #[prop(default = RwSignal::new(String::new()))] selected_container: RwSignal<String>,
    #[prop(default = RwSignal::new(String::new()))] selected_namespace: RwSignal<String>,
    #[prop(default = RwSignal::new(false))] follow_switch: RwSignal<bool>,
    #[prop(default = RwSignal::new(false))] previous_switch: RwSignal<bool>,
    #[prop(default = RwSignal::new(String::new()))] prompt_value: RwSignal<String>,
    actions: &'static [ActionType],
) -> impl IntoView {
    view(
        resource_type,
        namespace_name,
        resource_name,
        selected_container,
        selected_namespace,
        follow_switch,
        previous_switch,
        prompt_value,
        actions,
    )
}

fn view(
    resource_type: RwSignal<String>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    selected_container: RwSignal<String>,
    selected_namespace: RwSignal<String>,
    follow_switch: RwSignal<bool>,
    previous_switch: RwSignal<bool>,
    prompt_value: RwSignal<String>,
    actions: &'static [ActionType],
) -> impl IntoView {
    let label_namespace = if namespace_name.get_untracked().is_empty() {
        String::new()
    } else {
        format!(" • {}", namespace_name.get_untracked())
    };
    let label_resource = if resource_name.get_untracked().is_empty() {
        String::new()
    } else {
        format!(" • {}", resource_name.get_untracked())
    };
    let label = RwSignal::new(format!(
        "{}{label_namespace}{label_resource}",
        resource_type.get_untracked()
    ));

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="actions">
                    <div class="actions-label">{ label.get() }</div>
                    <Show when=move || actions.contains(&ActionType::NamespacesFilter)>
                        <namespaces::NamespacesFilterAction selected_namespace />
                    </Show>
                    <Show when=move || actions.contains(&ActionType::ContainersFilter)>
                        <containers::ContainersFilterAction
                            selected_container
                            resource_type
                            namespace_name
                            resource_name />
                    </Show>
                    <Show when=move || actions.contains(&ActionType::Follow)>
                        <follow::FollowAction follow_switch />
                    </Show>
                    <Show when=move || actions.contains(&ActionType::Previous)>
                        <previous::PreviousAction previous_switch />
                    </Show>
                    <Spacer />
                    <Show when=move || actions.contains(&ActionType::Logs)>
                        <logs::LogsAction
                            namespace_name
                            resource_name />
                    </Show>
                    <Show when=move || actions.contains(&ActionType::Scale)>
                        <scale::ScaleAction
                            resource_type
                            namespace_name
                            resource_name />
                    </Show>
                    <Show when=move || actions.contains(&ActionType::Edit)>
                        <edit::EditAction
                            resource_type
                            namespace_name
                            resource_name />
                    </Show>
                    <Show when=move || actions.contains(&ActionType::Delete)>
                        <delete::DeleteAction
                            resource_type
                            namespace_name
                            resource_name />
                    </Show>
                    <Show when=move || actions.contains(&ActionType::Prompt)>
                        <prompt::PromptAction prompt_value />
                    </Show>
                    <Show when=move || actions.contains(&ActionType::Save)>
                        <save::SaveAction
                            selected_container
                            resource_type
                            namespace_name
                            resource_name
                            previous_switch />
                    </Show>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}

pub fn or_none(data: RwSignal<String>) -> RwSignal<Option<String>> {
    let data = if data.get_untracked().is_empty() {
        None
    } else {
        Some(data.get_untracked())
    };
    RwSignal::new(data)
}
