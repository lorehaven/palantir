//! The create/edit-as-YAML modal.
//!
//! Shared by the nav "+" button (`method = "post"`, empty `initial_yaml`)
//! and every resource's Edit action (`method = "put"`, `initial_yaml`
//! pre-fetched server-side - there's no client-side fetch-on-open anymore,
//! so the textarea is never empty for a moment the way the old Leptos
//! dialog's was).
//!
//! Submits to the single generic `/ui/apply` route (see
//! `server::routes::apply`) - the payload's own `kind`/`metadata` decide
//! which K8s resource it targets, so this dialog never needs to know that
//! itself.

use quench_web::framework::dom::toggle_modal;
use quench_web::prelude::*;

pub fn apply_yaml_dialog(
    overlay_class: &str,
    panel_class: &str,
    initial_yaml: &str,
    method: &str,
    submit_url: &str,
) -> Element {
    let hide = toggle_modal(overlay_class, panel_class, "show");
    let textarea_id = format!("{overlay_class}-yaml");
    let method_attr = if method == "put" { "hx-put" } else { "hx-post" };

    div()
        .child(
            div()
                .class(format!("dialog-overlay {overlay_class}"))
                .attr("onclick", hide.clone()),
        )
        .child(
            div().class(format!("dialog-wrapper {panel_class}")).child(
                div()
                    .class("dialog apply-yaml-dialog")
                    .child(
                        div().class("dialog-content").child(
                            textarea()
                                .attr("id", textarea_id.clone())
                                .attr("name", "yaml")
                                .text(initial_yaml),
                        ),
                    )
                    .child(
                        div()
                            .class("dialog-footer")
                            .child(span().attr("style", "flex: 1"))
                            .child(
                                button()
                                    .class("btn btn-primary")
                                    .attr(method_attr, submit_url)
                                    .attr("hx-include", format!("#{textarea_id}"))
                                    .attr("hx-target", "body")
                                    .attr("hx-swap", "none")
                                    .text("Apply"),
                            )
                            .child(
                                button()
                                    .class("btn btn-primary")
                                    .attr("onclick", hide)
                                    .text("Cancel"),
                            ),
                    ),
            ),
        )
}
