use quench_web::prelude::*;

use crate::components::shared::table::{table, TableColumn, TableRow};
use crate::components::shared::wrapper::wrapper;

pub fn data_list_view(columns: &[TableColumn], rows: &[TableRow]) -> Element {
    wrapper(
        "",
        div()
            .class("card-container dcc-1")
            .child(div().class("card-table").child(table(columns, rows))),
    )
}

pub fn resource_info_view(data: &[(String, String)]) -> Element {
    let rows = data
        .iter()
        .fold(div().class("card-list"), |el, (name, value)| {
            el.child(
                div()
                    .class("card-list-row")
                    .child(div().class("card-list-row-title").text(name))
                    .child(div().class("card-list-row-content").text(value)),
            )
        });

    div().class("card-container dcc-1").child(rows)
}

pub fn obscured_resource_info_view(data: &[(String, String)]) -> Element {
    let rows = data.iter().enumerate().fold(
        div().class("card-list"),
        |el, (idx, (name, value))| {
            let visible_id = format!("obscured-{idx}");
            el.child(
                div()
                    .class("card-list-row-obscured")
                    .child(div().class("card-list-row-title").text(name))
                    .child(
                        div().class("card-list-row-button").child(
                            i().class("fa-solid fa-lock")
                                .attr("id", format!("{visible_id}-icon"))
                                .attr(
                                    "onclick",
                                    format!(
                                        "const c = document.getElementById('{visible_id}-content'); \
                                         const i = document.getElementById('{visible_id}-icon'); \
                                         const shown = c.classList.toggle('card-list-row-content'); \
                                         c.classList.toggle('card-list-row-content-obscured', !shown); \
                                         i.classList.toggle('fa-lock', !shown); \
                                         i.classList.toggle('fa-lock-open', shown);"
                                    ),
                                ),
                        ),
                    )
                    .child(
                        div()
                            .class("card-list-row-content-obscured")
                            .attr("id", format!("{visible_id}-content"))
                            .text(value),
                    ),
            )
        },
    );

    div().class("card-container dcc-1").child(rows)
}
