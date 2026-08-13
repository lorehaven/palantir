pub mod containers;
pub mod delete;
pub mod edit;
pub mod exec;
pub mod follow;
pub mod logs;
pub mod namespaces;
pub mod previous;
pub mod prompt;
pub mod save;
pub mod scale;

use quench_web::prelude::*;

use crate::components::shared::spacer::spacer;
use crate::components::shared::wrapper::wrapper;

/// Toolbar wrapper: label on the left, then whatever action elements the
/// page passes in, in order.
///
/// Callers build each action (`prompt::prompt_action`, `delete::delete_action`,
/// ...) themselves and pass the finished `Element`s - unlike the old Leptos
/// version there's no `ActionType` dispatch here, since there's no shared
/// reactive state (signals) for this toolbar to thread through to each
/// action anymore.
pub fn actions(label: &str, items: Vec<Element>) -> Element {
    let toolbar = items.into_iter().fold(
        div()
            .class("actions")
            .child(div().class("actions-label").text(label)),
        Element::child,
    );

    wrapper("", toolbar)
}

/// Convenience for a toolbar with a `Spacer` before the trailing (right-
/// aligned) actions, matching the old layout's fixed slot order (filters/
/// toggles, spacer, then buttons).
pub fn actions_with_trailing(
    label: &str,
    leading: Vec<Element>,
    trailing: Vec<Element>,
) -> Element {
    let mut items = leading;
    items.push(spacer());
    items.extend(trailing);
    actions(label, items)
}
