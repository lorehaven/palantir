use api::service_entries::get_service_entries;
use domain::workload::service::ServiceEntry;
use quench_cache::CacheStore;
use quench_web::prelude::*;

pub async fn render(cache: &CacheStore, current_path: &str) -> String {
    crate::shell::page(
        &["Services"],
        current_path,
        div()
            .class("facade content-facade")
            .child(fragment(cache).await),
    )
}

pub async fn fragment(cache: &CacheStore) -> Element {
    let entries = get_service_entries(cache).await.unwrap_or_default();

    entries
        .into_iter()
        .fold(div().class("entries"), |el, entry| {
            el.child(entry_link(&entry))
        })
        .attr("id", "facade-entries")
        .attr(
            "hx-get",
            format!("{}/facade/fragment", crate::base_path::ui_base()),
        )
        .attr("hx-trigger", "every 10s")
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

fn entry_link(entry: &ServiceEntry) -> Element {
    let class = if entry.available {
        "entry"
    } else {
        "entry-disabled"
    };
    let icon = if entry.available {
        i().class("fa-regular fa-circle-check")
            .attr("style", "font-size: 1.4rem;")
    } else {
        i().class("fa-regular fa-circle-xmark")
            .attr("style", "color: #bb0000; font-size: 1.4rem;")
    };

    let mut link = a().class(class).attr("target", "blank");
    if entry.available {
        link = link.attr("href", entry.url.clone());
    }

    link.child(
        div()
            .class("column-left")
            .child(div().class("entry-title").text(&entry.name))
            .child(div().text(&entry.url_display)),
    )
    .child(div().class("column-right").child(icon))
}
