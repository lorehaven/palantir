use domain::shared::response::Response;
use domain::workload::ingress::Ingress;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetIngresses, "/api/workloads/ingresses")]
pub async fn get_ingresses(namespace_name: Option<String>) -> Result<Vec<Ingress>, ServerFnError> {
    let response = resource_api::get("Ingress".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Ingress>>(&response)?.items)
}
