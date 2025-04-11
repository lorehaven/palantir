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
    let prompt = RwSignal::new(String::new());
    let selected = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Service Accounts"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="service-accounts main-page">
                    <Filter
                        label="Service Accounts"
                        selected
                        prompt
                        with_namespace=true
                        with_prompt=true />
                    <AccountsList selected prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}

#[component]
fn AccountsList(selected: RwSignal<String>, prompt: RwSignal<String>) -> impl IntoView {
    let accounts = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(3_600_000, move || {
        update_page_list(selected, prompt, accounts);
    });
    clear_page_effect(interval_handle);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 1),
        TableColumn::new("Name", TableColumnType::Link, 2),
        TableColumn::new("Age", TableColumnType::String, 1),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/cluster/namespaces/";
    params[2] = "/accounts/:1/serviceaccounts/";
    data_list_view(columns, accounts, styles, params)
}

fn update_page_list(
    namespace_name: RwSignal<String>,
    account_name: RwSignal<String>,
    accounts: RwSignal<Vec<Vec<String>>>,
) {
    if namespace_name.is_disposed() || account_name.is_disposed() {
        return;
    }
    let selected_value = namespace_name.get();
    let prompt_value = account_name.get();

    spawn_local(async move {
        let selected_value = if selected_value == "All Namespaces" {
            None
        } else {
            Some(selected_value)
        };
        let accounts_data = accounts_api::get_serviceaccounts(selected_value)
            .await
            .unwrap_or_default();

        accounts.set(
            accounts_data
                .into_iter()
                .filter(|n| {
                    n.metadata
                        .name
                        .to_lowercase()
                        .contains(&prompt_value.to_lowercase())
                })
                .map(|sa| {
                    vec![
                        "ServiceAccount".to_string(),
                        sa.clone().metadata.namespace,
                        sa.clone().metadata.name,
                        time_until_now(&sa.metadata.creation_timestamp.unwrap_or_default()),
                    ]
                })
                .collect(),
        );
    });
}
