use domain::account::bindings::{BaseRoleBinding, ClusterRoleBinding, RoleBinding};
use domain::shared::response::Response;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_all_bindings(cache: &CacheStore) -> Vec<Box<dyn BaseRoleBinding>> {
    let mut all_bindings = vec![];
    let bindings = get_rolebindings(cache, None)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn BaseRoleBinding>);
    let clusterbindings = get_clusterrolebindings(cache)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn BaseRoleBinding>);
    all_bindings.extend(bindings);
    all_bindings.extend(clusterbindings);
    all_bindings
}

pub async fn get_rolebindings(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<RoleBinding>> {
    let response = resource_api::get(cache, "RoleBinding", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<RoleBinding>>(&response)?.items)
}

pub async fn get_clusterrolebindings(
    cache: &CacheStore,
) -> anyhow::Result<Vec<ClusterRoleBinding>> {
    let response = resource_api::get(cache, "ClusterRoleBinding", None, None).await?;
    Ok(serde_json::from_str::<Response<ClusterRoleBinding>>(&response)?.items)
}
