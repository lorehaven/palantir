use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const CONTAINER_ID: &str = "pod-logs-container";
const FOLLOW_ID: &str = "pod-logs-follow";
const PREVIOUS_ID: &str = "pod-logs-previous";
const FILTER_ID: &str = "pod-logs-filter";
const CONTENT_ID: &str = "pod-logs-content";

pub async fn render(
    cache: &CacheStore,
    current_path: &str,
    namespace: &str,
    name: &str,
    container: &str,
    previous: bool,
    filter: &str,
) -> String {
    let containers = container_names(cache, namespace, name).await;
    let container = if container.is_empty() {
        containers.first().cloned().unwrap_or_default()
    } else {
        container.to_string()
    };
    let download_url = format!(
        "{}/workloads/{namespace}/pods/{name}/logs/download?container={container}&previous={previous}",
        crate::base_path::ui_base()
    );

    crate::shell::page(
        &["Workloads", namespace, "Pod", name],
        current_path,
        div()
            .class("workloads-pod-logs main-page")
            .child(actions(
                "Pod",
                vec![
                    containers_filter_action(CONTAINER_ID, &containers, &container),
                    follow_action(FOLLOW_ID, true),
                    previous_action(PREVIOUS_ID, previous),
                    prompt_action(FILTER_ID, filter),
                    save_action(&download_url),
                ],
            ))
            .child(fragment(cache, namespace, name, &container, previous, filter).await)
            .child(follow_script()),
    )
}

pub async fn fragment(
    cache: &CacheStore,
    namespace: &str,
    name: &str,
    container: &str,
    previous: bool,
    filter: &str,
) -> Element {
    let tail_lines = if previous { -1 } else { 100 };
    let logs = api::resource::logs(
        cache,
        "Pod",
        namespace.to_string(),
        name.to_string(),
        container.to_string(),
        previous,
        tail_lines,
    )
    .await
    .unwrap_or_default();

    let lines = logs
        .into_iter()
        .filter(|entry| entry.to_lowercase().contains(&filter.to_lowercase()));
    let content = lines.fold(pre(), |el, line| el.child(div().text(line)));

    // No periodic poll while viewing a previous (terminated) container's
    // logs - there's nothing new to arrive, and re-fetching its full
    // (`tail_lines=-1`) history every few seconds would be wasteful.
    let trigger = if previous {
        format!("change from:#{CONTAINER_ID}, change from:#{PREVIOUS_ID}, keyup changed delay:300ms from:#{FILTER_ID}")
    } else {
        format!("every 5s, change from:#{CONTAINER_ID}, change from:#{PREVIOUS_ID}, keyup changed delay:300ms from:#{FILTER_ID}")
    };

    // `id`/`hx-*` all live on the same element that's actually swapped
    // (outerHTML), so `follow_script`'s `evt.target.id` check below matches
    // it directly - wrapping it in `wrapper::wrapper` would put the id on a
    // different element than the one htmx swaps.
    div()
        .class("wrapper-container")
        .child(div().class("bar"))
        .child(
            div()
                .class("wrapper-content")
                .child(div().class("logs-view").child(content)),
        )
        .attr("id", CONTENT_ID)
        .attr(
            "hx-get",
            format!(
                "{}/workloads/{namespace}/pods/{name}/logs/fragment",
                crate::base_path::ui_base()
            ),
        )
        .attr("hx-trigger", trigger)
        .attr(
            "hx-include",
            format!("#{CONTAINER_ID}, #{PREVIOUS_ID}, #{FILTER_ID}"),
        )
        .attr("hx-target", "this")
        .attr("hx-swap", "outerHTML")
}

/// Auto-scrolls the log view to its newest line after each poll, but only
/// while `#pod-logs-follow` is checked - the closest vanilla-JS equivalent
/// of the old Leptos version's `Effect` that watched `data`/`follow_switch`
/// together.
///
/// `.logs-view` (not the swapped element itself) is what actually scrolls -
/// see `styles/logs.scss` - so this looks it up inside the swapped subtree
/// rather than assuming `evt.target` is scrollable.
fn follow_script() -> Element {
    // `.raw()` is required: `script()`'s text content is otherwise
    // HTML-entity-escaped like any other text node (`&&` -> `&amp;&amp;`,
    // `'` -> `&#39;`), and `<script>` bodies don't get entity-decoded by
    // the browser the way attribute values do - that would corrupt the JS.
    script(format!(
        "function scrollLogsToEnd() {{\
            var follow = document.getElementById('{FOLLOW_ID}');\
            var view = document.getElementById('{CONTENT_ID}');\
            view = view && view.querySelector('.logs-view');\
            if (follow && follow.checked && view) {{ view.scrollTop = view.scrollHeight; }}\
        }}\
        document.addEventListener('DOMContentLoaded', scrollLogsToEnd);\
        document.body.addEventListener('htmx:afterSwap', function(evt) {{\
            if (evt.target && evt.target.id === '{CONTENT_ID}') {{ scrollLogsToEnd(); }}\
        }});"
    ))
    .raw()
}
