use domain::shared::response::Response;
use domain::workload::job::Job;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetJobs, "/api/workloads/jobs")]
pub async fn get_jobs(namespace_name: Option<String>) -> Result<Vec<Job>, ServerFnError> {
    let response = resource_api::get("Job".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Job>>(&response)?.items)
}
