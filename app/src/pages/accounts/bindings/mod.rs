use leptos::prelude::*;

use crate::components::prelude::*;

mod bindings_list;

#[component]
pub fn AccountsRoleBindingsPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Accounts", "Role Bindings"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-bindings main-page">
                    <Filter
                        label="Role Bindings"
                        resource_name
                        with_resource_name=true />
                    <bindings_list::BindingsListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
