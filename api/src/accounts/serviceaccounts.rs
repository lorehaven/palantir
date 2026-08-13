use domain::account::serviceaccount::ServiceAccount;
use domain::shared::response::Response;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_serviceaccounts(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<ServiceAccount>> {
    let response = resource_api::get(cache, "ServiceAccount", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<ServiceAccount>>(&response)?.items)
}
