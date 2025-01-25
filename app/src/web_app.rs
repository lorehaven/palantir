use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::pages::{facade::FacadePage, dashboard::DashboardPage};

#[component]
pub fn WebApp() -> impl IntoView {
    provide_meta_context();

    view! {
        <Link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css" rel="stylesheet"/>
        <Stylesheet href="/pkg/palantir.css"/>
        <Title text="Palantir"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=DashboardPage/>
                    <Route path=StaticSegment("/facade") view=FacadePage/>
                </Routes>
            </main>
        </Router>
    }
}
