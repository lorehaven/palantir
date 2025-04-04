use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_request;
#[allow(unused_imports)]
use crate::domain::pod::{Pod, PodsResponse};

pub async fn get_pods_filtered(
    namespace_name: Option<String>,
    node_name: Option<String>,
) -> Vec<Pod> {
    if let Some(name) = namespace_name {
        get_pods_by_namespace_name(name.clone()).await.unwrap_or_default()
    } else if let Some(name) = node_name {
        get_pods_by_node_name(name.clone()).await.unwrap_or_default()
    } else {
        get_pods().await.unwrap_or_default()
    }
}

#[server(GetPods, "/api/pods")]
pub async fn get_pods() -> Result<Vec<Pod>, ServerFnError> {
    let response = kube_api_request("pods".to_string()).await?;
    Ok(serde_json::from_str::<PodsResponse>(&response)?.items)
}

#[server(GetPodsByNamespaceName, "/api/pods/by-namespace/:namespace_name")]
pub async fn get_pods_by_namespace_name(namespace_name: String) -> Result<Vec<Pod>, ServerFnError> {
    let response = kube_api_request("pods".to_string()).await?;
    let pods = serde_json::from_str::<PodsResponse>(&response)?.items
        .into_iter()
        .filter(|p| p.metadata.namespace == namespace_name)
        .collect();
    Ok(pods)
}

#[server(GetPodsByNodeName, "/api/pods/by-node/:node_name")]
pub async fn get_pods_by_node_name(node_name: String) -> Result<Vec<Pod>, ServerFnError> {
    let response = kube_api_request("pods".to_string()).await?;
    let pods = serde_json::from_str::<PodsResponse>(&response)?.items
        .into_iter()
        .filter(|p| p.spec.node_name == node_name)
        .collect();
    Ok(pods)
}
