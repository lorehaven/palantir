use leptos::prelude::*;

#[component]
pub fn CardList(
    labels: &'static[&'static str],
    widths: &'static[&'static str],
    rows: Vec<Vec<String>>,
) -> impl IntoView {
    view! {
        <div class="card-list">
            <div class="table-header">
                <For
                    each=move || labels.iter().zip(widths).clone()
                    key=|(l, _)| l.to_string()
                    children=move |(l, w)| view! {
                        <div class="table-header-item" style=format!("width: {}", w.to_string())>{ l.to_string() }</div>
                    }
                />
            </div>
            <div class="table-body">
                <For
                    each=move || rows.clone()
                    key=|r| r.join(" ")
                    children=move |r| view! {
                        <div class="table-row">
                            <For
                                each=move || r.clone().into_iter().zip(widths).clone()
                                key=|(r, _)| r.clone()
                                children=move |(r, w)| view! {
                                    <div class="table-row-item" style=format!("width: {}", w.to_string())>{ r.to_string() }</div>
                                }
                            />
                        </div>
                    }
                />
            </div>
        </div>
    }
}
