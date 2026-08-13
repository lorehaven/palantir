use quench_auth::prelude::{Claims, JwtConfig};
use quench_cache::CacheStore;
use quench_web::prelude::*;

use crate::components::prelude::*;

const TERMINAL_ID: &str = "exec-terminal";
const INPUT_ID: &str = "exec-input";

/// Unlike every other container-scoped page, the container `<select>` here
/// triggers a full page navigation rather than an htmx swap.
///
/// Switching containers means minting a fresh exec ticket and opening a new
/// WebSocket, which is simplest done by just re-rendering the whole page
/// (`onchange` sets `location.href`).
pub async fn render(
    cache: &CacheStore,
    config: &JwtConfig,
    claims: Option<&Claims>,
    current_path: &str,
    namespace: &str,
    name: &str,
    container: &str,
) -> String {
    let containers = container_names(cache, namespace, name).await;
    let container = if container.is_empty() {
        containers.first().cloned().unwrap_or_default()
    } else {
        container.to_string()
    };

    let nav_url = format!(
        "{}/workloads/{namespace}/pods/{name}/exec",
        crate::base_path::ui_base()
    );
    let container_select = containers.iter().fold(
        select().class("action containers-action").attr(
            "onchange",
            format!("location.href = '{nav_url}?container=' + encodeURIComponent(this.value)"),
        ),
        |el, c| {
            let mut opt = option().text(c);
            if *c == container {
                opt = opt.attr("selected", "selected");
            }
            el.child(opt)
        },
    );

    let terminal = if container.is_empty() {
        div()
            .class("exec-view")
            .child(div().text("No containers found for this pod."))
    } else {
        match api::ws_ticket::mint(
            cache,
            config,
            claims,
            namespace.to_string(),
            name.to_string(),
            container.clone(),
        )
        .await
        {
            Ok(ticket) => exec_terminal(&ticket),
            Err(err) => div()
                .class("exec-view")
                .child(div().text(format!("Could not start exec session: {err}"))),
        }
    };

    crate::shell::page(
        &["Workloads", namespace, "Pod", name],
        current_path,
        div()
            .class("workloads-pod-exec main-page")
            .child(actions("Pod", vec![container_select]))
            .child(terminal),
    )
}

fn exec_terminal(ticket: &str) -> Element {
    let ws_path = format!("{}/ws/exec?ticket={ticket}", crate::base_path::base_path());

    div()
        .class("exec-view")
        .child(div().class("terminal").attr("id", TERMINAL_ID))
        .child(
            div()
                .class("prompt")
                .child(span().text(">> "))
                .child(input().attr("id", INPUT_ID)),
        )
        // `.raw()` is required here too - see the comment on
        // `pages::workloads::pod_logs::follow_script` for why an unescaped
        // `<script>` body is necessary.
        .child(
            script(format!(
                "(function() {{\
                    var term = document.getElementById('{TERMINAL_ID}');\
                    var input = document.getElementById('{INPUT_ID}');\
                    var proto = location.protocol === 'https:' ? 'wss:' : 'ws:';\
                    var ws = new WebSocket(proto + '//' + location.host + '{ws_path}');\
                    ws.onmessage = function(evt) {{\
                        term.textContent += evt.data;\
                        term.scrollTop = term.scrollHeight;\
                    }};\
                    input.addEventListener('keyup', function(ev) {{\
                        if (ev.key === 'Enter' && input.value) {{\
                            ws.send(input.value);\
                            input.value = '';\
                        }}\
                    }});\
                }})();"
            ))
            .raw(),
        )
}
