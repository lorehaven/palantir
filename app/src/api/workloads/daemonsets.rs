use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_apps_request;
#[allow(unused_imports)]
use crate::domain::workload::daemonset::{Daemonset, DaemonsetsResponse};

#[server(GetDaemonsets, "/api/workloads/daemonsets")]
pub async fn get_daemonsets(
    namespace_name: Option<String>,
) -> Result<Vec<Daemonset>, ServerFnError> {
    let response = kube_api_apps_request("daemonsets".to_string()).await?;
    let items = serde_json::from_str::<DaemonsetsResponse>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
