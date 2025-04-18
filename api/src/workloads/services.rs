use domain::shared::response::Response;
use domain::workload::service::Service;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetServices, "/api/workloads/services")]
pub async fn get_services(namespace_name: Option<String>) -> Result<Vec<Service>, ServerFnError> {
    let response = resource_api::get("Service".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Service>>(&response)?.items)
}
