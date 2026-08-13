use domain::cluster::namespace::Namespace;
use domain::shared::response::Response;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_namespaces_response(cache: &CacheStore) -> anyhow::Result<Response<Namespace>> {
    let response = resource_api::get(cache, "Namespace", None, None).await?;
    Ok(serde_json::from_str::<Response<Namespace>>(&response)?)
}

pub async fn get_namespaces(cache: &CacheStore) -> anyhow::Result<Vec<Namespace>> {
    let response = resource_api::get(cache, "Namespace", None, None).await?;
    Ok(serde_json::from_str::<Response<Namespace>>(&response)?.items)
}
