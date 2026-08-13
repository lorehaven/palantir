use quench_web::prelude::*;

/// A filter/search input.
///
/// Carries no htmx wiring of its own - whatever polling fragment it should
/// filter listens for it via `hx-trigger="... from:#{id}"` and pulls its
/// value in via `hx-include` (see e.g. `pages::cluster::namespaces::fragment`),
/// so one input can drive a fetch on its own `keyup` without needing to know
/// the target itself.
pub fn prompt_action(id: &str, value: &str) -> Element {
    input()
        .attr("id", id)
        .attr("name", "name")
        .attr("type", "text")
        .attr("value", value)
        .attr("placeholder", "filter")
        .class("action prompt-action")
}
