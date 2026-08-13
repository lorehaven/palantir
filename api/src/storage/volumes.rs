use domain::shared::response::Response;
use domain::storage::volume::PersistentVolume;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_volumes(cache: &CacheStore) -> anyhow::Result<Vec<PersistentVolume>> {
    let response = resource_api::get(cache, "PersistentVolume", None, None).await?;
    Ok(serde_json::from_str::<Response<PersistentVolume>>(&response)?.items)
}
