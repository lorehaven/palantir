use leptos::prelude::*;

#[component]
pub fn ConfirmDialog<F>(
    confirm_label: RwSignal<String>,
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
                <div class="dialog confirm-dialog">
                    <div class="dialog-content">
                        <div>{confirm_label.get()}</div>
                        <div>Are you sure?</div>
                    </div>
                    <div class="dialog-footer">
                        <span style="flex: 1" />
                        <button class="btn btn-primary" on:click=move |_| yes(show_dialog, callback)>Yes</button>
                        <button class="btn btn-primary" on:click=move |_| no(show_dialog)>No</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

fn yes<F>(show_dialog: RwSignal<bool>, callback: RwSignal<F>)
where
    F: FnOnce() + Clone + Send + Sync + 'static,
{
    callback.get()();
    show_dialog.set(false);
}

fn no(show_dialog: RwSignal<bool>) {
    show_dialog.set(false);
}
