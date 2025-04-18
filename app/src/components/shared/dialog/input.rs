use leptos::prelude::*;

#[component]
pub fn InputDialog<F>(
    label: RwSignal<String>,
    value: RwSignal<i64>,
    show_dialog: RwSignal<bool>,
    callback: F,
) -> impl IntoView
where
    F: FnOnce() + Clone + Send + Sync + 'static,
{
    let callback = RwSignal::new(callback);

    view! {
        <Show when=move || show_dialog.get()>
            <div class="dialog-overlay" />
            <div class="dialog-wrapper">
                <div class="dialog input-dialog">
                    <div class="dialog-header">
                        <div>{label.get()}</div>
                    </div>
                    <div class="dialog-content">
                        <input
                            r#type="number"
                            min="0"
                            prop:value=value
                            on:keyup=move |ev| value.set(event_target_value(&ev).parse().unwrap())
                            prop:placeholder="filter"
                        />
                    </div>
                    <div class="dialog-footer">
                        <span style="flex: 1" />
                        <button class="btn btn-primary" on:click=move |_| apply(show_dialog, callback)>Apply</button>
                        <button class="btn btn-primary" on:click=move |_| cancel(show_dialog)>Cancel</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

fn apply<F>(show_dialog: RwSignal<bool>, callback: RwSignal<F>)
where
    F: FnOnce() + Clone + Send + Sync + 'static,
{
    callback.get()();
    show_dialog.set(false);
}

fn cancel(show_dialog: RwSignal<bool>) {
    show_dialog.set(false);
}
