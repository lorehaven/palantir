use leptos::prelude::*;

use crate::components::prelude::*;

mod roles_list;

#[component]
pub fn AccountsRolesPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Accounts", "Roles"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-roles main-page">
                    <Filter
                        label="Roles"
                        prompt
                        with_prompt=true />
                    <roles_list::RolesListComponent prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
