use domain::account::roles::{BaseRole, ClusterRole, Role};
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

pub async fn get_all_roles() -> Vec<Box<dyn BaseRole>> {
    let mut all_roles = vec![];
    let roles = get_roles(None)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn BaseRole>);
    let clusterroles = get_clusterroles()
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn BaseRole>);
    all_roles.extend(roles);
    all_roles.extend(clusterroles);
    all_roles
}

#[server(GetRoles, "/api/accounts/roles")]
pub async fn get_roles(namespace_name: Option<String>) -> Result<Vec<Role>, ServerFnError> {
    let response = resource_api::get("Role".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Role>>(&response)?.items)
}

#[server(GetClusterRoles, "/api/accounts/clusterroles")]
pub async fn get_clusterroles() -> Result<Vec<ClusterRole>, ServerFnError> {
    let response = resource_api::get("ClusterRole".to_string(), None, None).await?;
    Ok(serde_json::from_str::<Response<ClusterRole>>(&response)?.items)
}
