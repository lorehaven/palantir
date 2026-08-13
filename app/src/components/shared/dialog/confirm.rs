//! A confirm/cancel modal.
//!
//! Shown by toggling `overlay_class`/`panel_class` via
//! `quench_web::framework::dom::toggle_modal` from a trigger elsewhere on
//! the page (there's no reactive `show_dialog` signal anymore - the dialog
//! markup is always in the DOM, hidden by CSS until toggled).
//!
//! Confirming issues `hx-delete confirm_url`; on success the handler
//! responds with an `HX-Redirect` header (htmx then navigates there) rather
//! than a body swap - see the handler this posts to.
//!
//! Only one instance of a given `overlay_class`/`panel_class` pair may exist
//! per page (`toggle_modal` targets it by class, expecting exactly one
//! match) - fine for a single detail-page action, but a per-row dialog on a
//! list page needs a row-unique class pair.

use quench_web::framework::dom::toggle_modal;
use quench_web::prelude::*;

pub fn confirm_dialog(
    message: &str,
    overlay_class: &str,
    panel_class: &str,
    confirm_url: &str,
) -> Element {
    let hide = toggle_modal(overlay_class, panel_class, "show");

    div()
        .child(
            div()
                .class(format!("dialog-overlay {overlay_class}"))
                .attr("onclick", hide.clone()),
        )
        .child(
            div().class(format!("dialog-wrapper {panel_class}")).child(
                div()
                    .class("dialog confirm-dialog")
                    .child(
                        div()
                            .class("dialog-content")
                            .child(div().text(message))
                            .child(div().text("Are you sure?")),
                    )
                    .child(
                        div()
                            .class("dialog-footer")
                            .child(span().attr("style", "flex: 1"))
                            .child(
                                button()
                                    .class("btn btn-primary")
                                    .attr("hx-delete", confirm_url)
                                    .attr("hx-target", "body")
                                    .attr("hx-swap", "none")
                                    .text("Yes"),
                            )
                            .child(
                                button()
                                    .class("btn btn-primary")
                                    .attr("onclick", hide)
                                    .text("No"),
                            ),
                    ),
            ),
        )
}
