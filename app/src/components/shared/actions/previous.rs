use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn PreviousAction(previous_switch: RwSignal<bool>) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action previous-action">
                    <div class="actions-checkbox" on:click=move |_| previous_switch.set(!previous_switch.get())>
                        <input type="checkbox" checked=previous_switch />
                        <span class="slider" />
                    </div>
                    <div>Previous</div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
