use quench_cache::CacheStore;
use quench_web::framework::dom::toggle_modal;
use quench_web::prelude::*;

use crate::components::shared::dialog::apply_yaml::apply_yaml_dialog;

pub async fn edit_action(
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
    let yaml = json_to_yaml(&json);

    let overlay_class = "edit-yaml-overlay";
    let panel_class = "edit-yaml-panel";
    let submit_url = format!("{}/apply", crate::base_path::ui_base());

    div()
        .class("action edit-action")
        .child(apply_yaml_dialog(
            overlay_class,
            panel_class,
            &yaml,
            "put",
            &submit_url,
        ))
        .child(
            div()
                .class("actions-icon")
                .attr("onclick", toggle_modal(overlay_class, panel_class, "show"))
                .child(i().class("fa-solid fa-pen")),
        )
        .child(div().text("Edit"))
}

fn json_to_yaml(json: &str) -> String {
    serde_json::from_str::<serde_json::Value>(json)
        .ok()
        .and_then(|value| serde_yaml::to_string(&value).ok())
        .unwrap_or_default()
}
