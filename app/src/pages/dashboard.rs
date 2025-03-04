use leptos::prelude::*;

use crate::components::prelude::*;
use crate::pages::utils::shared::text::capitalize;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let entries = vec![
        ("cluster", "cube"),
        ("workloads", "rotate-right"),
        ("storage", "warehouse"),
        ("accounts", "users"),
        ("profile", "user"),
        ("apply", "plus"),
    ];

    view! {
        <Header text=vec![""] />
        <PageContent additional_classes="content-dashboard">
            <PageContentSlot slot>
                <div class="dashboard">
                    <div class="entries">
                        {entries.clone().into_iter()
                            .map(|item|
                                view! {
                                    <a href={format!("/{}", item.0)} class:entry>
                                        <div class="row">
                                            <i class=format!("fa-solid fa-{}", item.1) style="font-size: 1.4rem;" />
                                            <div class="entry-title">{{ capitalize(item.0) }}</div>
                                        </div>
                                    </a>
                                })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}
