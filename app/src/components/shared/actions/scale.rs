use quench_cache::CacheStore;
use quench_web::framework::dom::toggle_modal;
use quench_web::prelude::*;

use crate::components::shared::dialog::input::input_dialog;

pub async fn scale_action(
    cache: &CacheStore,
    resource_type: &str,
    namespace: Option<&str>,
    resource_name: &str,
) -> Element {
    let json = api::resource::get(
        cache,
        resource_type,
        namespace.map(String::from),
        Some(resource_name.to_string()),
    )
    .await
    .unwrap_or_default();
    let current_replicas = serde_json::from_str::<serde_json::Value>(&json)
        .ok()
        .and_then(|v| v["spec"]["replicas"].as_i64())
        .unwrap_or(0);

    let overlay_class = "scale-input-overlay";
    let panel_class = "scale-input-panel";
    let submit_url = format!("{}/scale", crate::base_path::ui_base());
    let hx_vals = serde_json::json!({
        "resource_type": resource_type,
        "namespace": namespace,
        "name": resource_name,
    })
    .to_string();

    div()
        .class("action scale-action")
        .child(input_dialog(
            overlay_class,
            panel_class,
            "Desired count",
            current_replicas,
            &submit_url,
            &hx_vals,
        ))
        .child(
            div()
                .class("actions-icon")
                .attr("onclick", toggle_modal(overlay_class, panel_class, "show"))
                .child(i().class("fa-solid fa-layer-group")),
        )
        .child(div().text("Scale"))
}
