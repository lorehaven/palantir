use api::apply as apply_api;
use api::utils::ApiMode;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn ApplyYamlDialog(
    show_dialog: RwSignal<bool>,
    #[prop(default = RwSignal::new(None))] resource: RwSignal<Option<String>>,
    mode: ApiMode,
) -> impl IntoView {
    let yaml_content = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());

    let interval_handle = update_page_effect(3_600_000, move || {
        update_resource(show_dialog, yaml_content, resource);
    });
    clear_page_effect(interval_handle);

    view! {
        <Show when=move || show_dialog.get()>
            <div class="dialog-overlay" />
            <div class="dialog-wrapper">
                <div class="dialog apply-yaml-dialog">
                    <div class="dialog-content">
                        <textarea bind:value={yaml_content} on:input=move |_| validate(yaml_content.get(), error) />
                    </div>
                    <div class="dialog-footer">
                        <span style="flex: 1" />
                        <button class="btn btn-primary" on:click=move |_| apply(yaml_content, show_dialog, error, mode)>Apply</button>
                        <button class="btn btn-primary" on:click=move |_| cancel(yaml_content, show_dialog, error)>Cancel</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

fn update_resource(
    show_dialog: RwSignal<bool>,
    yaml_content: RwSignal<String>,
    resource: RwSignal<Option<String>>,
) {
    if resource.is_disposed() {
        return;
    }

    let resource_value = resource.get();
    let show_dialog_value = show_dialog.get();

    spawn_local(async move {
        if show_dialog_value {
            if let Some(res) = resource_value {
                yaml_content.set(res);
                resource.set(None);
            }
        }
    });
}

fn validate(yaml_content: String, error: RwSignal<String>) {
    if let Err(err) = serde_yaml::from_str::<serde_yaml::Value>(&yaml_content) {
        error.set(err.to_string());
    } else {
        error.set(String::new());
    }
}

fn apply(
    yaml_content: RwSignal<String>,
    show_dialog: RwSignal<bool>,
    error: RwSignal<String>,
    mode: ApiMode,
) {
    let toaster = expect_toaster();
    let yaml_value = serde_yaml::from_str::<serde_yaml::Value>(&yaml_content.get()).unwrap();
    let json_value = serde_json::to_value(yaml_value).unwrap();
    let json_str = serde_json::to_string_pretty(&json_value).unwrap();

    spawn_local(async move {
        if let Err(err) = apply_api::apply(json_str.clone(), mode).await {
            toaster.error(err.to_string());
            match err {
                ServerFnError::ServerError(e) => error.set(e),
                _ => error.set(err.to_string()),
            }
        } else {
            yaml_content.set(String::new());
            toaster.success("Successfully applied yaml");
            show_dialog.set(false);
        }
    });
}

fn cancel(yaml_content: RwSignal<String>, show_dialog: RwSignal<bool>, error: RwSignal<String>) {
    yaml_content.set(String::new());
    error.set(String::new());
    show_dialog.set(false);
}
