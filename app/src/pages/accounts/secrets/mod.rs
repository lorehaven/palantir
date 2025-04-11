use leptos::prelude::*;

use crate::components::prelude::*;

mod secrets_list;

#[component]
pub fn AccountsSecretsPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());
    let selected = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Accounts", "Secrets"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-secrets main-page">
                    <Filter
                        label="Secrets"
                        selected
                        prompt
                        with_namespace=true
                        with_prompt=true />
                    <secrets_list::SecretsListComponent selected prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
