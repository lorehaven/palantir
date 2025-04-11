use leptos::prelude::*;
use leptos::task::spawn_local;

use api::service_entries::get_service_entries;
use domain::workload::service::ServiceEntry;
use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn FacadePage() -> impl IntoView {
    let entries = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let interval_handle = update_page_effect(10_000, move || update_page(entries, loading));
    clear_page_effect(interval_handle);

    view(entries, loading)
}

fn update_page(entries: RwSignal<Vec<ServiceEntry>>, loading: RwSignal<bool>) {
    spawn_local(async move {
        entries.set(get_service_entries().await.unwrap_or_default());
        loading.set(false);
    });
}

fn view(entries: RwSignal<Vec<ServiceEntry>>, loading: RwSignal<bool>) -> impl IntoView {
    view! {
        <Header text=vec!["Services"] />
        <PageContent additional_classes="content-facade">
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
