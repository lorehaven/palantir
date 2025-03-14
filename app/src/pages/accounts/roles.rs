use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn AccountsRolesPage() -> impl IntoView {
    view! {
        <Header text=vec!["Accounts", "Roles"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-roles main-page">
                    Roles
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
