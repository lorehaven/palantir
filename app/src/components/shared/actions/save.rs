use quench_web::prelude::*;

/// A plain download link.
///
/// The server sets `Content-Disposition: attachment` on `download_url` (see
/// `server::routes::workloads::pod_logs_download`), so there's no need for
/// the old Leptos version's Blob+synthetic-`<a>`-click JS dance anymore.
pub fn save_action(download_url: &str) -> Element {
    div()
        .class("action save-action")
        .child(
            a().attr("href", download_url)
                .class("actions-icon")
                .child(i().class("fa-solid fa-download")),
        )
        .child(div().text("Save"))
}
