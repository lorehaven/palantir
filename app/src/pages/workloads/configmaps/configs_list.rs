use api::workloads::configmaps as configs_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn ConfigsListComponent(selected: RwSignal<String>, prompt: RwSignal<String>) -> impl IntoView {
    let configs = RwSignal::new(vec![]);

    let interval_handle =
        update_page_effect(10_000, move || update_page(selected, prompt, configs));
    clear_page_effect(interval_handle);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 2),
        TableColumn::new("Name", TableColumnType::Link, 4),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/workloads/:1/configmaps/";
    data_list_view(columns, configs, styles, params)
}

fn update_page(
    namespace_name: RwSignal<String>,
    config_name: RwSignal<String>,
    configs: RwSignal<Vec<Vec<String>>>,
) {
    if namespace_name.is_disposed() || config_name.is_disposed() {
        return;
    }
    let selected_value = namespace_name.get();
    let config_name = config_name.get();

    spawn_local(async move {
        let selected_value = if selected_value == "All Namespaces" {
            None
        } else {
            Some(selected_value)
        };
        let configs_data = configs_api::get_configmaps(selected_value)
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|i| i.metadata.name.contains(&config_name))
            .collect::<Vec<_>>();

        let mut configs_vec = vec![];
        for config in configs_data {
            configs_vec.push(vec![
                "ConfigMap".to_string(),
                config.metadata.namespace,
                config.metadata.name,
            ]);
        }
        configs.set(configs_vec);
    });
}
