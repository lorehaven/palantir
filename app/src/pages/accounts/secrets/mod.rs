use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn AccountsSecretsPage() -> impl IntoView {
    view! {
        <Header text=vec!["Accounts", "Secrets"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-secrets main-page">
                    Secrets
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
