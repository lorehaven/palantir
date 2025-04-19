use codee::string::FromToStringCodec;
use leptos::attr::loading;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::{use_websocket, UseWebSocketReturn, core::ConnectionReadyState};
use web_sys::KeyboardEvent;
use crate::components::prelude::*;

#[component]
pub fn PodExecViewPage(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    selected_container: RwSignal<String>,
) -> impl IntoView {
    let content = RwSignal::new(String::new());
    let node_ref = NodeRef::<leptos::html::Div>::new();
    let command = RwSignal::new(String::new());
    let socket = RwSignal::new(None);
    let loading = RwSignal::new(true);

    Effect::new(move |_| {
        if selected_container.get().is_empty() {
            return;
        }

        let namespace_name = namespace_name.get();
        let resource_name = resource_name.get();
        let selected_container = selected_container.get();
        let ws_url = format!("/ws/exec?namespace={namespace_name}&pod={resource_name}&container={selected_container}");
        let ws = use_websocket::<String, String, FromToStringCodec>(&ws_url);
        socket.set(Some(ws));
    });

    Effect::new(move |_| {
        if socket.get().is_none() {
            return;
        }

        let socket = socket.get().unwrap();
        if socket.ready_state.get() != ConnectionReadyState::Open {
            return;
        }

        loading.set(false);
    });

    Effect::new(move |_| {
        if socket.get().is_none() {
            return;
        }

        let message = socket.get().unwrap().message;
        if let Some(msg) = message.get() {
            if !msg.is_empty() {
                content.update(|d| d.push_str(&msg));

                if let Some(container) = node_ref.get() {
                    container.set_scroll_top(container.scroll_height());
                }
            }
        }
    });

    let on_keyup = move |ev: KeyboardEvent| {
        let value = event_target_value(&ev);
        if ev.key_code() == 13 && !value.is_empty() {
            command.set(String::new());
            (socket.get().unwrap().send)(&value);
        }
    };

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="exec-view">
                    {move || if loading.get() {
                        view! {
                            <div class="loader" />
                        }.into_any()
                    } else {
                        view! {
                            <div class="terminal" node_ref=node_ref>{move || content.get()}</div>
                            <div class="prompt">
                                <span>">> "</span>
                                <input prop:value=command on:keyup=on_keyup />
                            </div>
                        }.into_any()
                    }}
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
