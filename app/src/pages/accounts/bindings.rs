use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn AccountsBindingsPage() -> impl IntoView {
    view! {
        <Header text=vec!["Accounts", "Bindings"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-bindings main-page">
                    Bindings
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
