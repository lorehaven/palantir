use api::resource as resource_api;
use api::utils::ApiMode;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::components::shared::dialog::apply_yaml::ApplyYamlDialog;

#[component]
pub fn EditAction(
    resource_type: RwSignal<String>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let show_dialog = RwSignal::new(false);
    let namespace_name = or_none(namespace_name);
    let resource_name = or_none(resource_name);
    let resource = RwSignal::new(None);

    Effect::new(move |_| {
        if show_dialog.get() {
            spawn_local(async move {
                let res = resource_api::get(
                    resource_type.get_untracked(),
                    namespace_name.get_untracked(),
                    resource_name.get_untracked(),
                )
                .await
                .unwrap_or_default();
                let yaml_value = serde_yaml::from_str::<serde_yaml::Value>(&res).unwrap();
                let yaml_value = serde_yaml::to_string(&yaml_value).unwrap();
                resource.set(Some(yaml_value));
            });
        }
    });

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action edit-action">
                    <ApplyYamlDialog show_dialog resource mode=ApiMode::Put />
                    <div class="actions-icon" on:click=move |_| show_dialog.set(true)>
                        <i class="fa-solid fa-pen" />
                    </div>
                    <div>Edit</div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
