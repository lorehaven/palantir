use leptos::prelude::*;

#[component]
pub fn TableComponent(
    columns: Vec<TableColumn>,
    values: Vec<Vec<String>>,
    styles: Vec<&'static str>,
    params: Vec<&'static str>,
) -> impl IntoView {
    let grid_template_columns = columns.iter()
        .map(|column| format!("{}fr", column.width))
        .collect::<Vec<String>>().join(" ");

    view! {
        <div class="table-header" style=format!("grid-template-columns: {grid_template_columns};")>
            {columns.clone().into_iter()
                .map(|item| view! { <div class="table-header-item"> { item.header.to_string() } </div> })
                .collect::<Vec<_>>()}
        </div>
        <div class="table-body" style=format!("grid-template-columns: {grid_template_columns};")>
            {values.clone().into_iter()
                .flatten()
                .enumerate()
                .map(|(idx, item)| {
                    let r#type = columns[idx % columns.len()].r#type.clone();
                    let style = styles[idx % styles.len()];
                    let param = params[idx % params.len()].to_string();
                    let values_idx = values[idx / values.first().unwrap_or(&vec!["".to_string()]).len()].clone();

                    let param = param.split("/")
                        .map(|p|
                            if p.starts_with(":") { values_idx[p.replace(":", "").parse::<usize>().unwrap_or(0)].clone().to_lowercase() }
                            else { p.to_string() })
                        .collect::<Vec<String>>()
                        .join("/");
                    leptos::logging::log!("{param}");
                    leptos::logging::log!("{values_idx:?}");

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
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

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
        Self { header: header.to_string(), r#type, width, }
    }
}

#[component]
fn BoolValue(item: String, style: &'static str) -> impl IntoView {
    let icon = if item == "true" { "check" } else { "xmark" };
    view! {
        <i class=format!("fa-regular fa-circle-{icon}") style=style />
    }
}

#[component]
fn LinkValue(item: String, style: &'static str, link: String) -> impl IntoView {
    view! {
        <a href=link class="table-body-item-link" style=style> { item.to_string() } </a>
    }
}

#[component]
fn StringValue(item: String, style: &'static str) -> impl IntoView {
    view! {
        <span class="table-body-item" style=style> { item.to_string() } </span>
    }
}

#[component]
fn StringListValue(item: String, style: &'static str) -> impl IntoView {
    view! {
        <ul class="table-body-item-list" style=style>
            {item.split("\n")
                .map(|item| view! { <li> { item.to_string() } </li> })
                .collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn StringTwoLineValue(item: String, style: &'static str) -> impl IntoView {
    view! {
        <ul class="table-body-item-two" style=style>
            {item.split("\n")
                .map(|item| view! { <li> { item.to_string() } </li> })
                .collect::<Vec<_>>()}
        </ul>
    }
}
