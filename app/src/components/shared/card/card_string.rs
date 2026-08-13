use quench_web::prelude::*;

pub fn card_string(label: &str, label_add: &str, value: &str) -> Element {
    div()
        .class("card-string")
        .child(
            div()
                .child(div().class("label").text(label))
                .child(div().class("label-add").text(label_add)),
        )
        .child(div().class("card-string-content").text(value))
}
