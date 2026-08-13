use domain::shared::response::Response;
use domain::workload::daemonset::DaemonSet;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_daemonsets(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<DaemonSet>> {
    let response = resource_api::get(cache, "DaemonSet", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<DaemonSet>>(&response)?.items)
}
