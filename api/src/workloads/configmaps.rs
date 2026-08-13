use domain::shared::response::Response;
use domain::workload::configmap::ConfigMap;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_configmaps(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<ConfigMap>> {
    let response = resource_api::get(cache, "ConfigMap", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<ConfigMap>>(&response)?.items)
}
