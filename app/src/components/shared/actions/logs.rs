use quench_web::prelude::*;

pub fn logs_action(namespace: &str, name: &str) -> Element {
    let url = format!(
        "{}/workloads/{namespace}/pods/{name}/logs",
        crate::base_path::ui_base()
    );

    div()
        .class("action logs-action")
        .child(
            a().attr("href", url)
                .class("actions-icon")
                .child(i().class("fa-solid fa-cloud-arrow-down")),
        )
        .child(div().text("Logs"))
}
