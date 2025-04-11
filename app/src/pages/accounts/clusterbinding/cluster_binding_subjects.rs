use api::accounts::bindings as bindings_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

#[component]
pub fn ClusterRoleBindingSubjectsComponent(cluster_binding_name: String) -> impl IntoView {
    let cluster_binding_name = RwSignal::new(cluster_binding_name);
    let cluster_binding_data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(60_000, move || {
        update_page(cluster_binding_name, cluster_binding_data)
    });
    clear_page_effect(interval_handle);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 3),
        TableColumn::new("Name", TableColumnType::String, 3),
        TableColumn::new("Namespace", TableColumnType::String, 3),
        TableColumn::new("Api Group", TableColumnType::String, 3),
    ];
    let styles = vec![""; columns.len()];
    let params = vec![""; columns.len()];
    data_list_view(columns, cluster_binding_data, styles, params)
}

fn update_page(
    cluster_binding_name: RwSignal<String>,
    cluster_binding_data: RwSignal<Vec<Vec<String>>>,
) {
    if cluster_binding_name.is_disposed() {
        return;
    }
    let cluster_binding_name = cluster_binding_name.get();

    spawn_local(async move {
        let crb = bindings_api::get_clusterrolebindings()
            .await
            .unwrap_or_default()
            .iter()
            .find(|sc| sc.metadata.name == cluster_binding_name)
            .cloned()
            .unwrap_or_default();

        cluster_binding_data.set(
            crb.subjects
                .into_iter()
                .map(|r| vec![r.kind, r.name, r.namespace, r.api_group])
                .collect(),
        );
    });
}
