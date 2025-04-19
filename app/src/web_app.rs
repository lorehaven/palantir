use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::prelude::*;
use crate::pages::accounts::binding::AccountsRoleBindingPage;
use crate::pages::accounts::bindings::AccountsRoleBindingsPage;
use crate::pages::accounts::clusterbinding::AccountsClusterRoleBindingPage;
use crate::pages::accounts::clusterrole::AccountsClusterRolePage;
use crate::pages::accounts::role::AccountsRolePage;
use crate::pages::accounts::roles::AccountsRolesPage;
use crate::pages::accounts::secret::AccountsSecretPage;
use crate::pages::accounts::secrets::AccountsSecretsPage;
use crate::pages::accounts::serviceaccount::ServiceAccountPage;
use crate::pages::accounts::AccountsPage;
use crate::pages::cluster::namespace::ClusterNamespacePage;
use crate::pages::cluster::namespaces::ClusterNamespacesPage;
use crate::pages::cluster::node::ClusterNodePage;
use crate::pages::cluster::nodes::ClusterNodesPage;
use crate::pages::cluster::ClusterPage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::facade::FacadePage;
use crate::pages::profile::ProfilePage;
use crate::pages::storage::claim::StorageClaimPage;
use crate::pages::storage::claims::StorageClaimsPage;
use crate::pages::storage::storageclass::StorageClassPage;
use crate::pages::storage::volume::StorageVolumePage;
use crate::pages::storage::volumes::StorageVolumesPage;
use crate::pages::storage::StorageClassesPage;
use crate::pages::workloads::configmap::WorkloadsConfigMapPage;
use crate::pages::workloads::configmaps::WorkloadsConfigMapsPage;
use crate::pages::workloads::deployment::WorkloadsDeploymentPage;
use crate::pages::workloads::ingress::WorkloadsIngressPage;
use crate::pages::workloads::ingresses::WorkloadsIngressesPage;
use crate::pages::workloads::job::WorkloadsJobPage;
use crate::pages::workloads::pod::WorkloadsPodPage;
use crate::pages::workloads::pod_logs::WorkloadsPodLogsPage;
use crate::pages::workloads::pods::WorkloadsPodsPage;
use crate::pages::workloads::replica::WorkloadsReplicaSetPage;
use crate::pages::workloads::replicas::WorkloadsReplicaSetsPage;
use crate::pages::workloads::service::WorkloadsServicePage;
use crate::pages::workloads::services::WorkloadsServicesPage;
use crate::pages::workloads::WorkloadsPage;

#[component]
pub fn WebApp() -> impl IntoView {
    let site_root = std::env::var("LEPTOS_SITE_PKG_DIR").unwrap_or_else(|_| "pkg".to_string());
    provide_toaster();
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
                    <Route path=path!("/workloads/:namespace/deployments/:name") view=WorkloadsDeploymentPage />
                    <Route path=path!("/workloads/:namespace/jobs/:name") view=WorkloadsJobPage />
                    <Route path=path!("/workloads/services") view=WorkloadsServicesPage />
                    <Route path=path!("/workloads/:namespace/services/:name") view=WorkloadsServicePage />
                    <Route path=path!("/workloads/replicas") view=WorkloadsReplicaSetsPage />
                    <Route path=path!("/workloads/:namespace/replicasets/:name") view=WorkloadsReplicaSetPage />
                    <Route path=path!("/workloads/pods") view=WorkloadsPodsPage />
                    <Route path=path!("/workloads/:namespace/pods/:name") view=WorkloadsPodPage />
                    <Route path=path!("/workloads/:namespace/pods/:name/logs") view=WorkloadsPodLogsPage />
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
                    <Route path=path!("/accounts/:namespace/roles/:name") view=AccountsRolePage />
                    <Route path=path!("/accounts/clusterroles/:name") view=AccountsClusterRolePage />
                    <Route path=path!("/accounts/bindings") view=AccountsRoleBindingsPage />
                    <Route path=path!("/accounts/:namespace/rolebindings/:name") view=AccountsRoleBindingPage />
                    <Route path=path!("/accounts/clusterrolebindings/:name") view=AccountsClusterRoleBindingPage />
                    <Route path=path!("/accounts/secrets") view=AccountsSecretsPage />
                    <Route path=path!("/accounts/:namespace/secrets/:name") view=AccountsSecretPage />
                    <Route path=path!("/profile") view=ProfilePage />
                </Routes>
            </main>
        </Router>
    }
}
