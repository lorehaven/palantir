use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_request;
#[allow(unused_imports)]
use crate::domain::shared::response::Response;
use crate::domain::cluster::namespace::Namespace;

#[server(GetNodesResponse, "/api/nodes/response")]
pub async fn get_namespaces_response() -> Result<Response<Namespace>, ServerFnError> {
    let response = kube_api_request("namespaces".to_string()).await?;
    Ok(serde_json::from_str::<Response<Namespace>>(&response)?)
}

#[server(GetEvents, "/api/namespaces")]
pub async fn get_namespaces() -> Result<Vec<Namespace>, ServerFnError> {
    let response = kube_api_request("namespaces".to_string()).await?;
    Ok(serde_json::from_str::<Response<Namespace>>(&response)?.items)
}
