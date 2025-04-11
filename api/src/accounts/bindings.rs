use domain::account::bindings::{BaseRoleBinding, ClusterRoleBinding, RoleBinding};
#[allow(unused_imports)]
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::utils::{kube_api_request, ApiType};

pub async fn get_all_bindings() -> Vec<Box<dyn BaseRoleBinding>> {
    let mut all_bindings = vec![];
    let bindings = get_rolebindings(None)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn BaseRoleBinding>);
    let clusterbindings = get_clusterrolebindings()
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn BaseRoleBinding>);
    all_bindings.extend(bindings);
    all_bindings.extend(clusterbindings);
    all_bindings
}

#[server(GetBindings, "/api/accounts/rolebindings")]
pub async fn get_rolebindings(
    namespace_name: Option<String>,
) -> Result<Vec<RoleBinding>, ServerFnError> {
    let response = kube_api_request(ApiType::Rbac, "rolebindings".to_string()).await?;
    let items = serde_json::from_str::<Response<RoleBinding>>(&response)?
        .items
        .into_iter()
        .filter(|f| {
            f.metadata
                .namespace
                .contains(&namespace_name.clone().unwrap_or_default())
        })
        .collect();
    Ok(items)
}

#[server(GetClusterBindings, "/api/accounts/clusterrolebindings")]
pub async fn get_clusterrolebindings() -> Result<Vec<ClusterRoleBinding>, ServerFnError> {
    let response = kube_api_request(ApiType::Rbac, "clusterrolebindings".to_string()).await?;
    Ok(serde_json::from_str::<Response<ClusterRoleBinding>>(&response)?.items)
}
