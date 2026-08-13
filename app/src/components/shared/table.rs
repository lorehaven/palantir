use quench_web::prelude::*;

pub type TableRow = (TableColumnType, String, String, String);

#[derive(Debug, Clone)]
pub enum TableColumnType {
    Bool,
    Link,
    String,
    StringList,
    StringTwoLine,
}

#[derive(Debug, Clone)]
pub struct TableColumn {
    header: String,
    r#type: TableColumnType,
    width: usize,
}

impl TableColumn {
    pub fn new(header: &'static str, r#type: TableColumnType, width: usize) -> Self {
        Self {
            header: header.to_string(),
            r#type,
            width,
        }
    }
}

pub fn parse_table_rows(
    columns: &[TableColumn],
    values: Vec<Vec<String>>,
    styles: &[String],
    params: &[String],
) -> Vec<TableRow> {
    values
        .clone()
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(idx, item)| {
            let r#type = columns[idx % columns.len()].r#type.clone();
            let style = styles[idx % styles.len()].clone();
            let param = params[idx % params.len()].clone();
            let values_idx = values[idx
                / values
                    .first()
                    .cloned()
                    .unwrap_or_else(|| vec![String::new()])
                    .len()]
            .clone();

            let re = regex::Regex::new(r":(\d+)").unwrap();
            let param = param
                .split('/')
                .map(|segment| {
                    re.replace_all(segment, |caps: &regex::Captures<'_>| {
                        let idx = caps[1].parse::<usize>().unwrap_or(0);
                        values_idx
                            .get(idx)
                            .cloned()
                            .unwrap_or_default()
                            .to_lowercase()
                    })
                    .to_string()
                })
                .filter(|segment| segment != "all namespaces")
                .collect::<Vec<String>>()
                .join("/");

            (r#type, item, param, style)
        })
        .collect::<Vec<_>>()
}

pub fn table(columns: &[TableColumn], rows: &[TableRow]) -> Element {
    let grid_template_columns = columns
        .iter()
        .map(|column| format!("{}fr", column.width))
        .collect::<Vec<String>>()
        .join(" ");

    let header = columns.iter().fold(
        div().class("table-header").attr(
            "style",
            format!("grid-template-columns: {grid_template_columns};"),
        ),
        |el, column| el.child(div().class("table-header-item").text(&column.header)),
    );

    let body = rows.iter().fold(
        div().class("table-body").attr(
            "style",
            format!("grid-template-columns: {grid_template_columns};"),
        ),
        |el, row| el.child(table_value(row)),
    );

    div().child(header).child(body)
}

fn table_value((r#type, item, param, style): &TableRow) -> Element {
    match r#type {
        TableColumnType::Bool => {
            let icon = if item == "true" { "check" } else { "xmark" };
            i().class(format!("fa-regular fa-circle-{icon}"))
                .attr("style", style)
        }
        TableColumnType::Link => {
            let link = format!("{}{param}{item}", crate::base_path::ui_base());
            a().attr("href", link)
                .class("table-body-item-link")
                .attr("style", style)
                .text(item)
        }
        TableColumnType::String => span()
            .class("table-body-item")
            .attr("style", style)
            .text(item),
        TableColumnType::StringList => item.split('\n').fold(
            ul().class("table-body-item-list").attr("style", style),
            |el, line| el.child(li().text(line)),
        ),
        TableColumnType::StringTwoLine => item.split('\n').fold(
            ul().class("table-body-item-two").attr("style", style),
            |el, line| el.child(li().text(line)),
        ),
    }
}
