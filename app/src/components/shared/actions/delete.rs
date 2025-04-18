use api::resource as resource_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::components::shared::dialog::confirm::ConfirmDialog;

#[component]
pub fn DeleteAction(
    resource_type: RwSignal<String>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let confirm_label = confirm_label(resource_type, namespace_name, resource_name);
    let show_dialog = RwSignal::new(false);
    let namespace_name = or_none(namespace_name);
    let resource_name = or_none(resource_name);

    let callback = move || {
        spawn_local(async move {
            if let Err(err) = resource_api::delete(
                    resource_type.get_untracked(),
                    namespace_name.get_untracked(),
                    resource_name.get_untracked(),
                ).await {
                Toast::error(&err.to_string());
            } else {
                show_dialog.set(false);
                if let Ok(history) = window().history() {
                    Toast::success("Resource successfully deleted");
                    let _ = history.back();
                }
            }
        });
    };

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action delete-action">
                    <ConfirmDialog
                        confirm_label
                        show_dialog
                        callback />
                    <div class="actions-icon" on:click=move |_| show_dialog.set(true)>
                        <i class="fa-solid fa-trash" />
                    </div>
                    <div>Delete</div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}

fn confirm_label(
    resource_type: RwSignal<String>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> RwSignal<String> {
    let type_label = resource_type.get_untracked().to_lowercase();
    let name_label = resource_name.get_untracked();
    let namespace_label =
        if namespace_name.get_untracked().is_empty() { String::new() }
        else { format!(" in `{}` namespace", namespace_name.get_untracked()) };
    RwSignal::new(format!("You are attempting to delete {type_label} `{name_label}`{namespace_label}"))
}
