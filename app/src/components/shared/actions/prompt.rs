use leptos::prelude::*;
use crate::components::prelude::*;

#[component]
pub fn PromptAction(
    prompt_value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action prompt-action">
                    <input
                        prop:value=prompt_value
                        on:keyup=move |ev| prompt_value.set(event_target_value(&ev).parse().unwrap())
                        prop:placeholder="filter"
                    />
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
