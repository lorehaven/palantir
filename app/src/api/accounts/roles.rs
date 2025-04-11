use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::{ApiType, kube_api_request};
#[allow(unused_imports)]
use crate::domain::shared::response::Response;
use crate::domain::account::roles::{BaseRole, Role, ClusterRole};

pub async fn get_all_roles() -> Vec<Box<dyn BaseRole>> {
    let mut all_roles = vec![];
    let roles = get_roles(None).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn BaseRole>);
    let clusterroles = get_clusterroles().await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn BaseRole>);
    all_roles.extend(roles);
    all_roles.extend(clusterroles);
    all_roles
}

#[server(GetRoles, "/api/accounts/roles")]
pub async fn get_roles(namespace_name: Option<String>) -> Result<Vec<Role>, ServerFnError> {
    let response = kube_api_request(ApiType::Rbac, "roles".to_string()).await?;
    let items = serde_json::from_str::<Response<Role>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}

#[server(GetClusterRoles, "/api/accounts/clusterroles")]
pub async fn get_clusterroles() -> Result<Vec<ClusterRole>, ServerFnError> {
    let response = kube_api_request(ApiType::Rbac, "clusterroles".to_string()).await?;
    Ok(serde_json::from_str::<Response<ClusterRole>>(&response)?.items)
}
