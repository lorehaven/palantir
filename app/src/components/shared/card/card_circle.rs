use quench_web::prelude::*;

pub fn card_circle(
    label: &str,
    label_add: &str,
    values: (f64, f64),
    value_labels: (&str, &str),
    decimal: bool,
) -> Element {
    let used_format = if decimal {
        format!("{:.0}", values.1)
    } else {
        format!("{:.2}", values.1)
    };
    let total_format = if decimal {
        format!("{:.0}", values.0)
    } else {
        format!("{:.2}", values.0)
    };
    let fill = (values.1 / values.0) * 100.0;

    div()
        .class("card-circle")
        .child(
            div()
                .child(div().class("label").text(label))
                .child(div().class("label-add").text(label_add)),
        )
        .child(
            div()
                .class("ring")
                .attr("style", format!("--fill: {fill}%"))
                .child(
                    div()
                        .class("ring-inner")
                        .child(
                            div()
                                .class("ring-inner-text")
                                .text(format!("{used_format} {}", value_labels.1)),
                        )
                        .child(div().class("ring-inner-text").text("of"))
                        .child(
                            div()
                                .class("ring-inner-text")
                                .text(format!("{total_format} {}", value_labels.0)),
                        ),
                ),
        )
}
