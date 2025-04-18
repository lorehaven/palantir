use domain::shared::response::Response;
use domain::workload::deployment::Deployment;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetDeployments, "/api/workloads/deployments")]
pub async fn get_deployments(
    namespace_name: Option<String>,
) -> Result<Vec<Deployment>, ServerFnError> {
    let response = resource_api::get("Deployment".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Deployment>>(&response)?.items)
}
