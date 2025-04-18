use domain::shared::response::Response;
use domain::workload::replicaset::ReplicaSet;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetReplicaSets, "/api/workloads/replicasets")]
pub async fn get_replicasets(
    namespace_name: Option<String>,
) -> Result<Vec<ReplicaSet>, ServerFnError> {
    let response = resource_api::get("ReplicaSet".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<ReplicaSet>>(&response)?.items)
}
