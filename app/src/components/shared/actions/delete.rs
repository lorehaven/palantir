use quench_web::framework::dom::toggle_modal;
use quench_web::prelude::*;

use crate::components::shared::dialog::confirm::confirm_dialog;

/// `confirm_url` is the `hx-delete` target the confirm dialog posts to (see
/// `dialog::confirm::confirm_dialog`) - the caller's actix route, which
/// responds with `HX-Redirect` on success.
pub fn delete_action(
    resource_type: &str,
    namespace: Option<&str>,
    resource_name: &str,
    confirm_url: &str,
) -> Element {
    let namespace_label = namespace
        .map(|ns| format!(" in `{ns}` namespace"))
        .unwrap_or_default();
    let message = format!(
        "You are attempting to delete {} `{resource_name}`{namespace_label}",
        resource_type.to_lowercase()
    );

    let overlay_class = "delete-confirm-overlay";
    let panel_class = "delete-confirm-panel";

    div()
        .class("action delete-action")
        .child(confirm_dialog(
            &message,
            overlay_class,
            panel_class,
            confirm_url,
        ))
        .child(
            div()
                .class("actions-icon")
                .attr("onclick", toggle_modal(overlay_class, panel_class, "show"))
                .child(i().class("fa-solid fa-trash")),
        )
        .child(div().text("Delete"))
}
