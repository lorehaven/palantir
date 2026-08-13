use quench_web::prelude::*;

/// Unlike `follow::follow_action`, toggling this one does need the server -
/// the log fragment listens for `change` on this checkbox's `id` and
/// re-fetches with `previous=true/false`.
pub fn previous_action(id: &str, checked: bool) -> Element {
    let mut checkbox = input()
        .attr("type", "checkbox")
        .attr("id", id)
        .attr("name", "previous");
    if checked {
        checkbox = checkbox.attr("checked", "checked");
    }

    div()
        .class("action previous-action")
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
        .child(div().text("Previous"))
}
