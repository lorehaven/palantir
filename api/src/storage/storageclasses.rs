use domain::shared::response::Response;
use domain::storage::storageclass::StorageClass;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_storageclasses(cache: &CacheStore) -> anyhow::Result<Vec<StorageClass>> {
    let response = resource_api::get(cache, "StorageClass", None, None).await?;
    Ok(serde_json::from_str::<Response<StorageClass>>(&response)?.items)
}
