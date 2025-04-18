use domain::cluster::namespace::Namespace;
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetNamespacesResponse, "/api/namespaces/response")]
pub async fn get_namespaces_response() -> Result<Response<Namespace>, ServerFnError> {
    let response = resource_api::get("Namespace".to_string(), None, None).await?;
    Ok(serde_json::from_str::<Response<Namespace>>(&response)?)
}

#[server(GetEvents, "/api/namespaces")]
pub async fn get_namespaces() -> Result<Vec<Namespace>, ServerFnError> {
    let response = resource_api::get("Namespace".to_string(), None, None).await?;
    Ok(serde_json::from_str::<Response<Namespace>>(&response)?.items)
}
