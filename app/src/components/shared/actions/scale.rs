use api::resource as resource_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::components::shared::dialog::input::InputDialog;

#[component]
pub fn ScaleAction(
    resource_type: RwSignal<String>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let label = RwSignal::new("Desired count".to_string());
    let value = RwSignal::new(0);
    let show_dialog = RwSignal::new(false);
    let namespace_name = or_none(namespace_name);
    let resource_name = or_none(resource_name);

    Effect::new(move |_| {
        spawn_local(async move {
            let res = resource_api::get(
                resource_type.get_untracked(),
                namespace_name.get_untracked(),
                resource_name.get_untracked(),
            )
                .await
                .unwrap_or_default();
            let json_value = serde_json::from_str::<serde_json::Value>(&res).unwrap();
            let replicas = json_value["spec"]["replicas"].as_i64().unwrap();
            value.set(replicas);
        });
    });

    let callback = move || {
        spawn_local(async move {
            if let Err(err) = resource_api::scale(
                resource_type.get_untracked(),
                namespace_name.get_untracked(),
                resource_name.get_untracked(),
                value.get(),
            ).await {
                Toast::error(&err.to_string());
            } else {
                show_dialog.set(false);
                Toast::success("Resource successfully re-scaled");
            }
        });
    };

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action scale-action">
                    <InputDialog
                        label
                        value
                        show_dialog
                        callback />
                    <div class="actions-icon" on:click=move |_| show_dialog.set(true)>
                        <i class="fa-solid fa-layer-group" />
                    </div>
                    <div>Scale</div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
