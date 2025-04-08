use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_request;
use crate::domain::cluster::namespace::Namespace;
#[allow(unused_imports)]
use crate::domain::cluster::namespace::NamespacesResponse;

#[server(GetNodesResponse, "/api/nodes/response")]
pub async fn get_namespaces_response() -> Result<NamespacesResponse, ServerFnError> {
    let response = kube_api_request("namespaces".to_string()).await?;
    Ok(serde_json::from_str::<NamespacesResponse>(&response)?)
}

#[server(GetEvents, "/api/namespaces")]
pub async fn get_namespaces() -> Result<Vec<Namespace>, ServerFnError> {
    let response = kube_api_request("namespaces".to_string()).await?;
    Ok(serde_json::from_str::<NamespacesResponse>(&response)?.items)
}
