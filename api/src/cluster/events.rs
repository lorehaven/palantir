use domain::cluster::event::Event;
use domain::shared::response::Response;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_events(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<Event>> {
    let response = resource_api::get(cache, "Event", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Event>>(&response)?.items)
}
