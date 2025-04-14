use api::accounts::serviceaccounts as accounts_api;
use domain::utils::time::time_until_now;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

pub mod serviceaccount;

pub mod binding;
pub mod bindings;
pub mod clusterbinding;
pub mod clusterrole;
pub mod role;
pub mod roles;
pub mod secret;
pub mod secrets;

#[component]
pub fn AccountsPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Service Accounts"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="service-accounts main-page">
                    <Filter
                        label="Service Accounts"
                        namespace_name
                        resource_name
                        with_namespace=true
                        with_resource_name=true />
                    <AccountsList namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}

#[component]
fn AccountsList(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let table_rows = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 1),
        TableColumn::new("Name", TableColumnType::Link, 2),
        TableColumn::new("Age", TableColumnType::String, 1),
    ];
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/cluster/namespaces/".to_string();
    params[2] = "/accounts/:1/serviceaccounts/".to_string();

    let columns_update = columns.clone();
    let interval_handle = update_page_effect(10_000, move || {
        update_page_list(
            columns_update.clone(),
            styles.clone(),
            params.clone(),
            table_rows,
            namespace_name,
            resource_name,
            loading,
        );
    });
    clear_page_effect(interval_handle);
    data_list_view(columns, table_rows, loading)
}

fn update_page_list(
    columns: Vec<TableColumn>,
    styles: Vec<String>,
    params: Vec<String>,
    table_rows: RwSignal<Vec<TableRow>>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    loading: RwSignal<bool>,
) {
    if namespace_name.is_disposed() || resource_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let resource_name = resource_name.get();

    spawn_local(async move {
        let list = update_page_list_async(
            columns.clone(),
            styles.clone(),
            params.clone(),
            namespace_name.clone(),
            resource_name.clone(),
        )
        .await
        .unwrap_or_default();
        table_rows.set(list);
        loading.set(false);
    });
}

#[server]
async fn update_page_list_async(
    columns: Vec<TableColumn>,
    styles: Vec<String>,
    params: Vec<String>,
    namespace_name: String,
    resource_name: String,
) -> Result<Vec<TableRow>, ServerFnError> {
    let namespace_name = if namespace_name == "All Namespaces" {
        None
    } else {
        Some(namespace_name)
    };
    let mut list = accounts_api::get_serviceaccounts(namespace_name)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|n| {
            n.metadata
                .name
                .to_lowercase()
                .contains(&resource_name.to_lowercase())
        })
        .map(|sa| {
            vec![
                "ServiceAccount".to_string(),
                sa.clone().metadata.namespace,
                sa.clone().metadata.name,
                time_until_now(&sa.metadata.creation_timestamp.unwrap_or_default()),
            ]
        })
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    Ok(parse_table_rows(columns, list, styles, params))
}
