use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn FollowAction(follow_switch: RwSignal<bool>) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action follow-action">
                    <div class="actions-checkbox" on:click=move |_| follow_switch.set(!follow_switch.get())>
                        <input type="checkbox" checked=follow_switch />
                        <span class="slider" />
                    </div>
                    <div>Follow</div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
