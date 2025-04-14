use leptos::prelude::*;

use crate::components::prelude::*;

mod roles_list;

#[component]
pub fn AccountsRolesPage() -> impl IntoView {
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Accounts", "Roles"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-roles main-page">
                    <Filter
                        label="Roles"
                        resource_name
                        with_resource_name=true />
                    <roles_list::RolesListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
