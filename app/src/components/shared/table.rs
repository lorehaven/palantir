use leptos::prelude::*;
use serde::{Deserialize, Serialize};

pub type TableRow = (TableColumnType, String, String, String);

pub fn parse_table_rows(
    columns: Vec<TableColumn>,
    values: Vec<Vec<String>>,
    styles: Vec<String>,
    params: Vec<String>,
) -> Vec<TableRow> {
    values
        .clone()
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(idx, item)| {
            let r#type = columns[idx % columns.len()].r#type.clone();
            let style = styles[idx % styles.len()].to_string();
            let param = params[idx % params.len()].to_string();
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

#[component]
pub fn TableComponent(
    columns: Vec<TableColumn>,
    table_rows: RwSignal<Vec<TableRow>>,
    loading: RwSignal<bool>,
) -> impl IntoView {
    let grid_template_columns = columns
        .iter()
        .map(|column| format!("{}fr", column.width))
        .collect::<Vec<String>>()
        .join(" ");

    view! {
        <div class="table-header" style=format!("grid-template-columns: {grid_template_columns};")>
            {columns.into_iter().map(|item| {
                view! { <div class="table-header-item"> { item.header } </div> }.into_any()
            }).collect::<Vec<_>>()}
        </div>
        {move || if loading.get() {
            view! { <div class="loader" /> }.into_any()
        } else {
            view! {
                <div class="table-body" style=format!("grid-template-columns: {grid_template_columns};")>
                    {table_rows.get().into_iter().map(|(r#type, item, param, style)| {
                        match r#type {
                            TableColumnType::Bool => view! { <BoolValue item style /> }.into_any(),
                            TableColumnType::Link => {
                                let link = format!("{param}{item}");
                                view! { <LinkValue item style link /> }.into_any()
                            },
                            TableColumnType::String => view! { <StringValue item style /> }.into_any(),
                            TableColumnType::StringList => view! { <StringListValue item style /> }.into_any(),
                            TableColumnType::StringTwoLine => view! { <StringTwoLineValue item style /> }.into_any(),
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }.into_any()
        }}
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TableColumnType {
    Bool,
    Link,
    String,
    StringList,
    StringTwoLine,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[component]
fn BoolValue(item: String, style: String) -> impl IntoView {
    let icon = if item == "true" { "check" } else { "xmark" };
    view! {
        <i class=format!("fa-regular fa-circle-{icon}") style=style />
    }
}

#[component]
fn LinkValue(item: String, style: String, link: String) -> impl IntoView {
    view! {
        <a href=link class="table-body-item-link" style=style> { item } </a>
    }
}

#[component]
fn StringValue(item: String, style: String) -> impl IntoView {
    view! {
        <span class="table-body-item" style=style> { item } </span>
    }
}

#[component]
fn StringListValue(item: String, style: String) -> impl IntoView {
    view! {
        <ul class="table-body-item-list" style=style>
            {item.split('\n')
                .map(|item| view! { <li> { item.to_string() } </li> })
                .collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn StringTwoLineValue(item: String, style: String) -> impl IntoView {
    view! {
        <ul class="table-body-item-two" style=style>
            {item.split('\n')
                .map(|item| view! { <li> { item.to_string() } </li> })
                .collect::<Vec<_>>()}
        </ul>
    }
}
