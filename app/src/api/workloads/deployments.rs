use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_apps_request;
#[allow(unused_imports)]
use crate::domain::shared::response::Response;
use crate::domain::workload::deployment::Deployment;

#[server(GetDeployments, "/api/workloads/deployments")]
pub async fn get_deployments(
    namespace_name: Option<String>,
) -> Result<Vec<Deployment>, ServerFnError> {
    let response = kube_api_apps_request("deployments".to_string()).await?;
    let items = serde_json::from_str::<Response<Deployment>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
