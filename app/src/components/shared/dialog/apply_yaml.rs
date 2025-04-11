use leptos::prelude::*;
use leptos::task::spawn_local;

use api::apply as apply_api;

#[component]
pub fn ApplyYamlDialog(
    show_dialog: RwSignal<bool>,
) -> impl IntoView {
    let yaml_content = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());

    view! {
        <Show when=move || show_dialog.get()>
            <div class="dialog-overlay" />
            <div class="dialog-wrapper">
                <div class="dialog apply-yaml-dialog">
                    <div class="dialog-content">
                        <textarea bind:value={yaml_content} on:input=move |_| validate(yaml_content.get(), error) />
                    </div>
                    <div class="dialog-footer">
                        <Show when=move || !error.get().is_empty()>
                            <div class="dialog-error">"Error: " {error.get()}</div>
                        </Show>
                        <span style="flex: 1" />
                        <button class="btn btn-primary" on:click=move |_| apply(yaml_content, show_dialog, error)>Apply</button>
                        <button class="btn btn-primary" on:click=move |_| cancel(yaml_content, show_dialog, error)>Cancel</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

fn validate(
    yaml_content: String,
    error: RwSignal<String>,
) {
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
) {
    let yaml_value = serde_yaml::from_str::<serde_yaml::Value>(&yaml_content.get()).unwrap();
    let json_value = serde_json::to_value(yaml_value).unwrap();
    let json_str = serde_json::to_string_pretty(&json_value).unwrap();

    spawn_local(async move {
        if let Err(err) = apply_api::apply(json_str.clone()).await { match err {
            ServerFnError::ServerError(e) => error.set(e),
            _ => error.set(err.to_string()),
        } } else {
            yaml_content.set(String::new());
            show_dialog.set(false);
        }
    });
}

fn cancel(
    yaml_content: RwSignal<String>,
    show_dialog: RwSignal<bool>,
    error: RwSignal<String>,
) {
    yaml_content.set(String::new());
    error.set(String::new());
    show_dialog.set(false);
}
