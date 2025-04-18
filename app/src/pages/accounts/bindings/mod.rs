use leptos::prelude::*;

use crate::components::prelude::*;

mod bindings_list;

#[component]
pub fn AccountsRoleBindingsPage() -> impl IntoView {
    let resource_type = RwSignal::new("RoleBindings".to_string());
    let resource_name = RwSignal::new(String::new());

    view! {
        <Header text=vec!["Accounts", "Role Bindings"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="accounts-bindings main-page">
                    <Actions
                        resource_type
                        prompt_value=resource_name
                        actions=&[ActionType::Prompt] />
                    <bindings_list::BindingsListComponent resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
