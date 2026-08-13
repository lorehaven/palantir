use quench_web::prelude::*;

pub fn wrapper(label: &str, content: Element) -> Element {
    div()
        .class("wrapper-container")
        .child(div().class("bar").text(label))
        .child(div().class("wrapper-content").child(content))
}
