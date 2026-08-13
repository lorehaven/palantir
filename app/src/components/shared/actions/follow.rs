use quench_web::prelude::*;

/// A pure client-side toggle.
///
/// No htmx wiring, since "follow" only controls whether the log view's own
/// inline script auto-scrolls after each poll (see
/// `pages::workloads::pod_logs`), not what gets fetched.
pub fn follow_action(id: &str, checked: bool) -> Element {
    let mut checkbox = input().attr("type", "checkbox").attr("id", id);
    if checked {
        checkbox = checkbox.attr("checked", "checked");
    }

    div()
        .class("action follow-action")
        .child(
            div()
                .class("actions-checkbox")
                .attr(
                    "onclick",
                    format!("document.getElementById('{id}').click()"),
                )
                .child(checkbox)
                .child(span().class("slider")),
        )
        .child(div().text("Follow"))
}
