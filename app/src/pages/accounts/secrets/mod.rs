use leptos::prelude::*;

use crate::components::prelude::*;

mod secrets_list;

#[component]
pub fn AccountsSecretsPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Accounts", "Secrets"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-secrets main-page">
                    <Filter
                        label="Secrets"
                        namespace_name
                        resource_name
                        with_namespace=true
                        with_resource_name=true />
                    <secrets_list::SecretsListComponent namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
