use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_request;
#[allow(unused_imports)]
use crate::domain::shared::response::Response;
use crate::domain::workload::service::Service;

#[server(GetServices, "/api/workloads/services")]
pub async fn get_services(
    namespace_name: Option<String>,
) -> Result<Vec<Service>, ServerFnError> {
    let response = kube_api_request("services".to_string()).await?;
    let items = serde_json::from_str::<Response<Service>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
