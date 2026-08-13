use domain::account::roles::{BaseRole, ClusterRole, Role};
use domain::shared::response::Response;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_all_roles(cache: &CacheStore) -> Vec<Box<dyn BaseRole>> {
    let mut all_roles = vec![];
    let roles = get_roles(cache, None)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn BaseRole>);
    let clusterroles = get_clusterroles(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn BaseRole>);
    all_roles.extend(roles);
    all_roles.extend(clusterroles);
    all_roles
}

pub async fn get_roles(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<Role>> {
    let response = resource_api::get(cache, "Role", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Role>>(&response)?.items)
}

pub async fn get_clusterroles(cache: &CacheStore) -> anyhow::Result<Vec<ClusterRole>> {
    let response = resource_api::get(cache, "ClusterRole", None, None).await?;
    Ok(serde_json::from_str::<Response<ClusterRole>>(&response)?.items)
}
