use domain::shared::response::Response;
use domain::workload::daemonset::DaemonSet;
use leptos::prelude::ServerFnError;
use leptos::server;
use crate::resource as resource_api;

#[server(GetDaemonSets, "/api/workloads/daemonsets")]
pub async fn get_daemonsets(
    namespace_name: Option<String>,
) -> Result<Vec<DaemonSet>, ServerFnError> {
    let response = resource_api::get("DaemonSet".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<DaemonSet>>(&response)?.items)
}
