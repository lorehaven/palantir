use quench_web::prelude::*;

pub fn render(current_path: &str) -> String {
    crate::shell::page(
        &[],
        current_path,
        div()
            .class("dashboard main-page")
            .child(p().text("Palantir is being migrated to quench-web - more pages land soon.")),
    )
}
