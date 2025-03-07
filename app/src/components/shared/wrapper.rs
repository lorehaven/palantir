use leptos::prelude::*;

#[slot]
pub struct WrapperSlot {
    children: ChildrenFn,
}

#[component]
pub fn Wrapper(
    wrapper_slot: WrapperSlot,
) -> impl IntoView {
    view! {
        <div>
            { move || { (wrapper_slot.children)().into_any() } }
        </div>
    }
}
