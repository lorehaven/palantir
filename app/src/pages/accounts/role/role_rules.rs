use api::accounts::roles as roles_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn RoleRulesComponent(namespace_name: String, role_name: String) -> impl IntoView {
    let namespace_name = RwSignal::new(namespace_name);
    let role_name = RwSignal::new(role_name);
    let role_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || {
        update_page(namespace_name, role_name, role_data)
    });
    clear_page_effect(interval_handle);

    let columns = vec![
        TableColumn::new("Groups", TableColumnType::StringList, 3),
        TableColumn::new("Resources", TableColumnType::StringList, 3),
        TableColumn::new("Verbs", TableColumnType::StringList, 3),
        TableColumn::new("Names", TableColumnType::StringList, 3),
    ];
    let styles = vec![""; columns.len()];
    let params = vec![""; columns.len()];
    data_list_view(columns, role_data, styles, params)
}

fn update_page(
    namespace_name: RwSignal<String>,
    role_name: RwSignal<String>,
    role_data: RwSignal<Vec<Vec<String>>>,
) {
    if role_name.is_disposed() || namespace_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let role_name = role_name.get();

    spawn_local(async move {
        let selected_value = if namespace_name == "All Namespaces" {
            None
        } else {
            Some(namespace_name)
        };
        let role = roles_api::get_roles(selected_value)
            .await
            .unwrap_or_default()
            .iter()
            .find(|sc| sc.metadata.name == role_name)
            .cloned()
            .unwrap_or_default();

        role_data.set(
            role.rules
                .into_iter()
                .map(|r| {
                    vec![
                        r.api_groups.join("\n"),
                        r.resources.join("\n"),
                        r.verbs.join("\n"),
                        r.resource_names.join("\n"),
                    ]
                })
                .collect(),
        );
    });
}
