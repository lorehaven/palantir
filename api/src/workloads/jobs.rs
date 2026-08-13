use domain::shared::response::Response;
use domain::workload::job::Job;
use quench_cache::CacheStore;

use crate::resource as resource_api;

pub async fn get_jobs(
    cache: &CacheStore,
    namespace_name: Option<String>,
) -> anyhow::Result<Vec<Job>> {
    let response = resource_api::get(cache, "Job", namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Job>>(&response)?.items)
}
