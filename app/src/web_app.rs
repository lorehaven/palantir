use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{components::{Route, Router, Routes}, path};

use crate::pages::{
    facade::FacadePage,
    dashboard::DashboardPage,
    cluster::{
        ClusterPage,
        namespace::ClusterNamespacePage,
        namespaces::ClusterNamespacesPage,
        node::ClusterNodePage,
        nodes::ClusterNodesPage,
    },
    workloads::{
        WorkloadsPage,
        service::WorkloadsServicePage,
        services::WorkloadsServicesPage,
        replica::WorkloadsReplicaSetPage,
        replicas::WorkloadsReplicaSetsPage,
        pod::WorkloadsPodPage,
        pods::WorkloadsPodsPage,
        ingress::WorkloadsIngressPage,
        ingresses::WorkloadsIngressesPage,
        configmap::WorkloadsConfigMapPage,
        configmaps::WorkloadsConfigMapsPage,
    },
    storage::{
        StorageClassesPage,
        storageclass::StorageClassPage,
        volume::StorageVolumePage,
        volumes::StorageVolumesPage,
        claim::StorageClaimPage,
        claims::StorageClaimsPage,
    },
    accounts::{
        AccountsPage,
        serviceaccount::ServiceAccountPage,
        clusterrole::ClusterRolePage,
        role::RolePage,
        roles::AccountsRolesPage,
        bindings::AccountsBindingsPage,
        secrets::AccountsSecretsPage,
    },
    profile::ProfilePage,
};

#[component]
pub fn WebApp() -> impl IntoView {
    let site_root = std::env::var("LEPTOS_SITE_PKG_DIR").unwrap_or_else(|_| "pkg".to_string());
    provide_meta_context();

    view! {
        <Link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css" rel="stylesheet"/>
        <Stylesheet href=format!("/pkg/palantir.css")/>
        <Stylesheet href=format!("/{site_root}/palantir.css")/>
        <Title text="Palantir"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/facade") view=FacadePage />
                    <Route path=path!("/") view=DashboardPage />
                    <Route path=path!("/cluster") view=ClusterPage />
                    <Route path=path!("/cluster/nodes") view=ClusterNodesPage />
                    <Route path=path!("/cluster/nodes/:name") view=ClusterNodePage />
                    <Route path=path!("/cluster/namespaces") view=ClusterNamespacesPage />
                    <Route path=path!("/cluster/namespaces/:name") view=ClusterNamespacePage />
                    <Route path=path!("/workloads") view=WorkloadsPage />
                    <Route path=path!("/workloads/services") view=WorkloadsServicesPage />
                    <Route path=path!("/workloads/:namespace/services/:name") view=WorkloadsServicePage />
                    <Route path=path!("/workloads/replicas") view=WorkloadsReplicaSetsPage />
                    <Route path=path!("/workloads/:namespace/replicasets/:name") view=WorkloadsReplicaSetPage />
                    <Route path=path!("/workloads/pods") view=WorkloadsPodsPage />
                    <Route path=path!("/workloads/:namespace/pods/:name") view=WorkloadsPodPage />
                    <Route path=path!("/workloads/ingresses") view=WorkloadsIngressesPage />
                    <Route path=path!("/workloads/:namespace/ingresses/:name") view=WorkloadsIngressPage />
                    <Route path=path!("/workloads/configmaps") view=WorkloadsConfigMapsPage />
                    <Route path=path!("/workloads/:namespace/configmaps/:name") view=WorkloadsConfigMapPage />
                    <Route path=path!("/storage") view=StorageClassesPage />
                    <Route path=path!("/storageclasses/:name") view=StorageClassPage />
                    <Route path=path!("/storage/volumes") view=StorageVolumesPage />
                    <Route path=path!("/storage/volumes/:name") view=StorageVolumePage />
                    <Route path=path!("/storage/claims") view=StorageClaimsPage />
                    <Route path=path!("/storage/:namespace/claims/:name") view=StorageClaimPage />
                    <Route path=path!("/accounts") view=AccountsPage />
                    <Route path=path!("/accounts/:namespace/serviceaccounts/:name") view=ServiceAccountPage />
                    <Route path=path!("/accounts/roles") view=AccountsRolesPage />
                    <Route path=path!("/accounts/:namespace/roles/:name") view=RolePage />
                    <Route path=path!("/accounts/clusterroles/:name") view=ClusterRolePage />
                    <Route path=path!("/accounts/bindings") view=AccountsBindingsPage />
                    <Route path=path!("/accounts/secrets") view=AccountsSecretsPage />
                    <Route path=path!("/profile") view=ProfilePage />
                </Routes>
            </main>
        </Router>
    }
}
