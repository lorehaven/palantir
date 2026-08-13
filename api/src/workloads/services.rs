use domain::shared::response::Response;
use domain::workload::service::Service;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_services(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<Service>> {
    let response = resource_api::get(cache, "Service", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Service>>(&response)?.items)
}
