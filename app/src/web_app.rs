use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{components::{Route, Router, Routes}, path};

use crate::pages::{
    cluster::{
        ClusterPage,
        namespaces::ClusterNamespacesPage,
        node::ClusterNodePage,
        nodes::ClusterNodesPage,
    },
    dashboard::DashboardPage,
    facade::FacadePage,
};
use crate::pages::cluster::namespace::ClusterNamespacePage;

#[component]
pub fn WebApp() -> impl IntoView {
    let site_root = std::env::var("LEPTOS_SITE_PKG_DIR").unwrap_or("pkg".to_string());
    provide_meta_context();

    view! {
        <Link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css" rel="stylesheet"/>
        <Stylesheet href=format!("/pkg/palantir.css")/>
        <Stylesheet href=format!("/{site_root}/palantir.css")/>
        <Title text="Palantir"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=DashboardPage />
                    <Route path=path!("/cluster") view=ClusterPage />
                    <Route path=path!("/cluster/nodes") view=ClusterNodesPage />
                    <Route path=path!("/cluster/nodes/:name") view=ClusterNodePage />
                    <Route path=path!("/cluster/namespaces") view=ClusterNamespacesPage />
                    <Route path=path!("/cluster/namespaces/:name") view=ClusterNamespacePage />
                    <Route path=path!("/facade") view=FacadePage />
                </Routes>
            </main>
        </Router>
    }
}
