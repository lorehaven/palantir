use quench_web::prelude::*;

pub fn exec_action(namespace: &str, name: &str) -> Element {
    let url = format!(
        "{}/workloads/{namespace}/pods/{name}/exec",
        crate::base_path::ui_base()
    );

    div()
        .class("action exec-action")
        .child(
            a().attr("href", url)
                .class("actions-icon")
                .child(i().class("fa-solid fa-terminal")),
        )
        .child(div().text("Exec"))
}
