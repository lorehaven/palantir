use leptos::prelude::*;

use crate::components::prelude::*;

mod roles_list;

#[component]
pub fn AccountsRolesPage() -> impl IntoView {
    let resource_type = RwSignal::new("Roles".to_string());
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Accounts", "Roles"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-roles main-page">
                    <Actions
                        resource_type
                        prompt_value=resource_name
                        actions=&[ActionType::Prompt] />
                    <roles_list::RolesListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
