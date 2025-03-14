use leptos::prelude::*;

use crate::components::prelude::*;

pub mod bindings;
pub mod roles;
pub mod secrets;

#[component]
pub fn AccountsPage() -> impl IntoView {
    view! {
        <Header text=vec!["Accounts"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts main-page">
                    Accounts
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
