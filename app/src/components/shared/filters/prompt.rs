use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn PromptFilter(
    prompt: RwSignal<String>,
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="prompt-filter">
                    <input
                        prop:value=prompt
                        on:keyup=move |ev| prompt.set(event_target_value(&ev).parse().unwrap())
                        prop:placeholder="type name"
                    />
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
