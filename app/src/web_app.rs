use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{components::{Route, Router, Routes}, path};

use crate::pages::{
    cluster::ClusterPage,
    cluster::nodes::ClusterNodesPage,
    cluster::namespaces::ClusterNamespacesPage,
    dashboard::DashboardPage,
    facade::FacadePage,
};

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
                    <Route path=path!("/") view=DashboardPage />
                    <Route path=path!("/cluster") view=ClusterPage />
                    <Route path=path!("/cluster/nodes") view=ClusterNodesPage />
                    <Route path=path!("/cluster/namespaces") view=ClusterNamespacesPage />
                    <Route path=path!("/facade") view=FacadePage />
                </Routes>
            </main>
        </Router>
    }
}
