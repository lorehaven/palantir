use domain::shared::response::Response;
use domain::workload::deployment::Deployment;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_deployments(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<Deployment>> {
    let response = resource_api::get(cache, "Deployment", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Deployment>>(&response)?.items)
}
