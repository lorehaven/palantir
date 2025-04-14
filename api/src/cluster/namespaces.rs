use domain::cluster::namespace::Namespace;
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::utils::{kube_api_request, ApiType};

#[server(GetNodesResponse, "/api/nodes/response")]
pub async fn get_namespaces_response() -> Result<Response<Namespace>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "namespaces".to_string()).await?;
    Ok(serde_json::from_str::<Response<Namespace>>(&response)?)
}

#[server(GetEvents, "/api/namespaces")]
pub async fn get_namespaces() -> Result<Vec<Namespace>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "namespaces".to_string()).await?;
    Ok(serde_json::from_str::<Response<Namespace>>(&response)?.items)
}
