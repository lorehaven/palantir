//! A single-number input modal (currently just `actions::scale::scale_action`).
//!
//! `hx_vals_json` carries whatever context the submit endpoint needs beyond
//! the number itself (resource type/namespace/name) as a static JSON blob -
//! htmx merges it into the submitted params alongside the input's own value.

use quench_web::framework::dom::toggle_modal;
use quench_web::prelude::*;

pub fn input_dialog(
    overlay_class: &str,
    panel_class: &str,
    label: &str,
    current_value: i64,
    submit_url: &str,
    hx_vals_json: &str,
) -> Element {
    let hide = toggle_modal(overlay_class, panel_class, "show");
    let input_id = format!("{overlay_class}-value");

    div()
        .child(
            div()
                .class(format!("dialog-overlay {overlay_class}"))
                .attr("onclick", hide.clone()),
        )
        .child(
            div().class(format!("dialog-wrapper {panel_class}")).child(
                div()
                    .class("dialog input-dialog")
                    .child(div().class("dialog-header").child(div().text(label)))
                    .child(
                        div().class("dialog-content").child(
                            input()
                                .attr("type", "number")
                                .attr("min", "0")
                                .attr("id", input_id.clone())
                                .attr("name", "replicas")
                                .attr("value", current_value.to_string()),
                        ),
                    )
                    .child(
                        div()
                            .class("dialog-footer")
                            .child(span().attr("style", "flex: 1"))
                            .child(
                                button()
                                    .class("btn btn-primary")
                                    .attr("hx-put", submit_url)
                                    .attr("hx-vals", hx_vals_json)
                                    .attr("hx-include", format!("#{input_id}"))
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
