use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::utils::{kube_api_request, ApiType};
#[allow(unused_imports)]
use domain::shared::response::Response;
use domain::workload::daemonset::DaemonSet;

#[server(GetDaemonSets, "/api/workloads/daemonsets")]
pub async fn get_daemonsets(
    namespace_name: Option<String>,
) -> Result<Vec<DaemonSet>, ServerFnError> {
    let response = kube_api_request(ApiType::Apps, "daemonsets".to_string()).await?;
    let items = serde_json::from_str::<Response<DaemonSet>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
