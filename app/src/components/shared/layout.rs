use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::pages::utils::shared::text::capitalize;

#[component]
pub fn Header(
    #[prop(optional)]
    text: Vec<impl Into<String>>,
) -> impl IntoView {
    let text = text.into_iter()
        .map(|t| t.into())
        .collect::<Vec<String>>();
    let mut links = text
        .iter()
        .filter(|t| !t.is_empty())
        .enumerate()
        .map(|(idx, t)| {
            if idx == text.len() - 1 {
                view! {
                    <span class="header-separator"> / </span>
                    <span class="header-link">{ t.to_string() }</span>
                }.into_any()
            } else {
                let href = format!("/{}", text[0..=idx].iter()
                    .map(|t| t.to_lowercase())
                    .collect::<Vec<String>>()
                    .join("/"));
                view! {
                    <span class="header-separator"> / </span>
                    <a href=href class="header-link">{ t.to_string() }</a>
                }.into_any()
            }
        })
        .collect::<Vec<_>>();
    links.insert(0, view! { <a href="/" class="header-link">Palantir</a> }.into_any());
    view! {
        <div class="header"> { links } </div>
    }
}

#[component]
pub fn SideNavBar() -> impl IntoView {
    let current_path = use_location().pathname.get_untracked();

    view! {
        <div class="side-nav-bar">
            <SideNavBarEntry name="cluster" icon="cube" />
            <SideNavBarEntry name="nodes" icon="hexagon-nodes" url_prefix="/cluster" visible={current_path.starts_with("/cluster")} />
            <SideNavBarEntry name="namespaces" icon="circle-nodes" url_prefix="/cluster" visible={current_path.starts_with("/cluster")} />
            <div class="separator" />
            <SideNavBarEntry name="workloads" icon="rotate-right" />
            <div class="separator" />
            <SideNavBarEntry name="storage" icon="warehouse" />
            <div class="separator" />
            <SideNavBarEntry name="accounts" icon="users" />
            <div class="separator" />
            <SideNavBarEntry name="profile" icon="user" />
            <div class="separator" />
            <SideNavBarEntry name="apply" icon="plus" />
        </div>
    }
}

#[component]
pub fn SideNavBarEntry(
    name: &'static str,
    icon: &'static str,
    #[prop(optional)]
    url_prefix: &'static str,
    #[prop(default = true)]
    visible: bool,
) -> impl IntoView {
    view! {
        <Show when=move || visible>
            <a href=format!("{}/{}", url_prefix, &name) class="side-nav-bar-entry">
                <i class=format!("fa-solid fa-{icon}") />
                <div class="side-nav-bar-entry-title">{{ capitalize(name) }}</div>
            </a>
        </Show>
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
pub fn PageContent(
    page_content_slot: PageContentSlot,
    #[prop(optional)]
    additional_classes: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("content {additional_classes}")>
            <SideNavBar />
            <div class="content-internal">
                { move || { (page_content_slot.children)().into_any() } }
            </div>
        </div>
    }
}
