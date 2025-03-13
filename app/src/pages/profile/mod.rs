use leptos::prelude::*;

use crate::components::prelude::*;

pub mod current_user;

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <Header text=vec!["Account", "Token"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="profile main-page">
                    <div class="profile-title">Current user</div>
                    <current_user::CurrentUserComponent />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}