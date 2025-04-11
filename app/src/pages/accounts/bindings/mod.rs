use leptos::prelude::*;

use crate::components::prelude::*;

mod bindings_list;

#[component]
pub fn AccountsRoleBindingsPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Accounts", "Role Bindings"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-bindings main-page">
                    <Filter
                        label="Role Bindings"
                        prompt
                        with_prompt=true />
                    <bindings_list::BindingsListComponent prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
