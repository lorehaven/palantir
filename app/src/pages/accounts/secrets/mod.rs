use leptos::prelude::*;

use crate::components::prelude::*;

mod secrets_list;

#[component]
pub fn AccountsSecretsPage() -> impl IntoView {
    let resource_type = RwSignal::new("Secrets".to_string());
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Accounts", "Secrets"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-secrets main-page">
                    <Actions
                        resource_type
                        selected_namespace=namespace_name
                        prompt_value=resource_name
                        actions=&[ActionType::NamespacesFilter, ActionType::Prompt] />
                    <secrets_list::SecretsListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
