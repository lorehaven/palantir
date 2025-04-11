use api::accounts::roles as roles_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn ClusterRoleRulesComponent(cluster_role_name: String) -> impl IntoView {
    let cluster_role_name = RwSignal::new(cluster_role_name);
    let cluster_role_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || {
        update_page(cluster_role_name, cluster_role_data)
    });
    clear_page_effect(interval_handle);

    let columns = vec![
        TableColumn::new("Groups", TableColumnType::StringList, 3),
        TableColumn::new("Resources", TableColumnType::StringList, 3),
        TableColumn::new("Non Resource", TableColumnType::StringList, 3),
        TableColumn::new("Verbs", TableColumnType::StringList, 3),
        TableColumn::new("Names", TableColumnType::StringList, 3),
    ];
    let styles = vec![""; columns.len()];
    let params = vec![""; columns.len()];
    data_list_view(columns, cluster_role_data, styles, params)
}

fn update_page(cluster_role_name: RwSignal<String>, cluster_role_data: RwSignal<Vec<Vec<String>>>) {
    if cluster_role_name.is_disposed() {
        return;
    }
    let cluster_role_name = cluster_role_name.get();

    spawn_local(async move {
        let cr = roles_api::get_clusterroles()
            .await
            .unwrap_or_default()
            .iter()
            .find(|sc| sc.metadata.name == cluster_role_name)
            .cloned()
            .unwrap_or_default();

        cluster_role_data.set(
            cr.rules
                .into_iter()
                .map(|r| {
                    vec![
                        r.api_groups.join("\n"),
                        r.resources.join("\n"),
                        String::new(),
                        r.verbs.join("\n"),
                        r.resource_names.join("\n"),
                    ]
                })
                .collect(),
        );
    });
}
