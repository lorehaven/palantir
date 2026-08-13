use domain::shared::response::Response;
use domain::workload::replicaset::ReplicaSet;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_replicasets(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<ReplicaSet>> {
    let response = resource_api::get(cache, "ReplicaSet", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<ReplicaSet>>(&response)?.items)
}
