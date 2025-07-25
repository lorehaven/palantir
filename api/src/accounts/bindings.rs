use domain::account::bindings::{BaseRoleBinding, ClusterRoleBinding, RoleBinding};
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

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
    let response = resource_api::get("RoleBinding".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<RoleBinding>>(&response)?.items)
}

#[server(GetClusterBindings, "/api/accounts/clusterrolebindings")]
pub async fn get_clusterrolebindings() -> Result<Vec<ClusterRoleBinding>, ServerFnError> {
    let response = resource_api::get("ClusterRoleBinding".to_string(), None, None).await?;
    Ok(serde_json::from_str::<Response<ClusterRoleBinding>>(&response)?.items)
}
