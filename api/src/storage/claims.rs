use domain::shared::response::Response;
use domain::storage::claim::PersistentVolumeClaim;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_claims(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<PersistentVolumeClaim>> {
    let response = resource_api::get(cache, "PersistentVolumeClaim", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<PersistentVolumeClaim>>(&response)?.items)
}
