use domain::shared::response::Response;
use domain::workload::ingress::Ingress;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_ingresses(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<Ingress>> {
    let response = resource_api::get(cache, "Ingress", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Ingress>>(&response)?.items)
}
