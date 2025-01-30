use leptos::prelude::*;

#[component]
pub fn Header(text: String) -> impl IntoView {
    view! {
        <div class="header">LoreHaven | Palantir{text}</div>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="footer">"© 2025 Paweł Walus - Order of Devs | LoreHaven"</div>
    }
}

#[slot]
pub struct PageContentSlot {
    children: ChildrenFn,
}

#[component]
pub fn PageContent(page_content_slot: PageContentSlot) -> impl IntoView {
    view! {
        <div class="content">
            { move || { (page_content_slot.children)().into_any() } }
        </div>
    }
}
