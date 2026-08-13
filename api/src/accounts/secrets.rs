use domain::account::secret::Secret;
use domain::shared::response::Response;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_secrets(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<Secret>> {
    let response = resource_api::get(cache, "Secret", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Secret>>(&response)?.items)
}
