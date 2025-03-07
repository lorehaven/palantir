use leptos::prelude::*;

#[slot]
pub struct WrapperSlot {
    children: ChildrenFn,
}

#[component]
pub fn Wrapper(
    #[prop(default = "")]
    label: &'static str,
    wrapper_slot: WrapperSlot,
) -> impl IntoView {
    view! {
        <div class="wrapper-container">
            <div class="bar">{label}</div>
            <div class="wrapper-content">
                { move || { (wrapper_slot.children)().into_any() } }
            </div>
        </div>
    }
}
