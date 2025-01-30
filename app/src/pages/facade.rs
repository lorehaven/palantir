use leptos::prelude::*;
use leptos::task::spawn;

use crate::api::services as api;
use crate::pages::components::prelude::*;

#[component]
pub fn FacadePage() -> impl IntoView {
    let entries = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    Effect::new(move |_| spawn(async move {
        entries.set(api::get_services().await.unwrap_or_default());
        loading.set(false);
    }));

    view! {
        <Header text=" > Services".to_string() />
        <PageContent>
            <PageContentSlot slot>
                <div class="facade">
                    <Show
                        when=move || { !loading.get() }
                        fallback=|| view! { <div class="loader" /> }>
                        <div class="entries">
                            <For
                                each=move || entries.get()
                                key=|item| item.name.clone()
                                children=move |item| {
                                    let enabled = item.available;
                                    view! {
                                        <a href=if enabled { Some(item.url) } else { None } target="blank"
                                            class:entry=move || enabled class:entry-disabled=move || !enabled>
                                            <div class="column-left">
                                                <div class="entry-title">{{ item.name }}</div>
                                                <div>{{ item.url_display }}</div>
                                            </div>
                                            <div class="column-right">
                                                <Show
                                                    when=move || enabled
                                                    fallback=|| view! {
                                                        <i class="fa-regular fa-circle-xmark" style="color: #bb0000; font-size: 1.4rem;" />
                                                    }>
                                                        <i class="fa-regular fa-circle-check" style="font-size: 1.4rem;" />
                                                </Show>
                                            </div>
                                        </a>
                                    }
                                }
                            />
                        </div>
                    </Show>
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
