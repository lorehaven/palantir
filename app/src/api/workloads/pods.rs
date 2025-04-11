use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_request;
#[allow(unused_imports)]
use crate::domain::shared::response::Response;
use crate::domain::cluster::pod::Pod;

#[server(GetPods, "/api/pods")]
pub async fn get_pods(
    namespace_name: Option<String>,
    node_name: Option<String>,
) -> Result<Vec<Pod>, ServerFnError> {
    let response = kube_api_request("pods".to_string()).await?;
    let pods = serde_json::from_str::<Response<Pod>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .filter(|f| f.spec.node_name.contains(&node_name.clone().unwrap_or_default()))
        .collect();
    Ok(pods)
}
