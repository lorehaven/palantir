use api::resource as resource_api;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::{HtmlAnchorElement, Url};
use wasm_bindgen::JsCast;

use crate::components::prelude::*;

#[component]
pub fn SaveAction(
    selected_container: RwSignal<String>,
    resource_type: RwSignal<String>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    previous_switch: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action save-action">
                    <div class="actions-icon" on:click=move |_| save(
                        selected_container,
                        resource_type,
                        namespace_name,
                        resource_name,
                        previous_switch,
                    )>
                        <i class="fa-solid fa-download" />
                    </div>
                    <div>Save</div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}

fn save(
    container: RwSignal<String>,
    resource_type: RwSignal<String>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    previous_switch: RwSignal<bool>,
) {
    spawn_local(async move {
        let container = container.get_untracked();
        let resource_type = resource_type.get_untracked();
        let namespace_name = namespace_name.get_untracked();
        let resource_name = resource_name.get_untracked();
        let previous_switch = previous_switch.get_untracked();

        let now = chrono::Local::now().naive_local();
        let filename = format!(
            "logs_{resource_name}_{}.log",
            now.format("%Y-%m-%d_%H-%M-%S-%3f")
        );

        let logs = resource_api::logs(
            resource_type,
            namespace_name,
            resource_name,
            container,
            previous_switch,
            -1,
        )
        .await
        .unwrap_or_default();

        let blob_parts = js_sys::Array::new();
        blob_parts.push(&wasm_bindgen::JsValue::from_str(&logs.join("\n")));
        let blob = leptos::web_sys::Blob::new_with_str_sequence(&blob_parts).unwrap();
        let url = Url::create_object_url_with_blob(&blob).unwrap();
        let window = window();
        let document = window.document().unwrap();
        let a = document
            .create_element("a")
            .unwrap()
            .dyn_into::<HtmlAnchorElement>()
            .unwrap();

        a.set_href(&url);
        a.set_download(&filename);

        document.body().unwrap().append_child(&a).unwrap();
        a.click();

        document.body().unwrap().remove_child(&a).unwrap();
        Url::revoke_object_url(&url).unwrap();
    });
}
