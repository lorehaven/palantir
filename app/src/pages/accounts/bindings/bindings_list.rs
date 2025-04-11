use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use api::accounts::bindings as bindings_api;

#[component]
pub fn BindingsListComponent(
    prompt: RwSignal<String>,
) -> impl IntoView {
    let bindings = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || update_page(bindings, prompt));
    clear_page_effect(interval_handle);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 2),
        TableColumn::new("Namespace", TableColumnType::String, 2),
        TableColumn::new("Name", TableColumnType::Link, 3),
        TableColumn::new("Age", TableColumnType::String, 1),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/accounts/:1/:0s/";
    data_list_view(columns, bindings, styles, params)
}

fn update_page(
    bindings: RwSignal<Vec<Vec<String>>>,
    prompt: RwSignal<String>,
) {
    if prompt.is_disposed() { return; }
    let prompt_value = prompt.get();

    spawn_local(async move {
        let bindings_list = bindings_api::get_all_bindings().await;
        let mut list = bindings_list.into_iter()
            .filter(|r| r.get_name().to_lowercase().contains(&prompt_value.to_lowercase()))
            .map(|r| r.to_model())
            .map(|r| vec![
                r.r#type, r.namespace, r.name, r.age,
            ]).collect::<Vec<_>>();
        list.sort_by(|a, b| a[2].cmp(&b[2]));
        bindings.set(list);
    });
}
