use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::{ApiType, kube_api_request};
#[allow(unused_imports)]
use crate::domain::shared::response::Response;
use crate::domain::workload::job::Job;

#[server(GetJobs, "/api/workloads/jobs")]
pub async fn get_jobs(
    namespace_name: Option<String>,
) -> Result<Vec<Job>, ServerFnError> {
    let response = kube_api_request(ApiType::Batch, "jobs".to_string()).await?;
    let items = serde_json::from_str::<Response<Job>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
