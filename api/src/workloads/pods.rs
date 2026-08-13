use domain::cluster::pod::Pod;
use domain::shared::response::Response;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_pods(
    cache: &CacheStore,
    namespace_name: Option<String>,
    node_name: Option<String>,
) -> anyhow::Result<Vec<Pod>> {
    let response = resource_api::get(cache, "Pod", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Pod>>(&response)?
        .items
        .into_iter()
        .filter(|f| {
            f.spec
                .node_name
                .contains(&node_name.clone().unwrap_or_default())
        })
        .collect())
}
