use domain::shared::response::Response;
use domain::workload::replicaset::ReplicaSet;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::utils::{kube_api_request, ApiType};

#[server(GetReplicaSets, "/api/workloads/replicasets")]
pub async fn get_replicasets(
    namespace_name: Option<String>,
) -> Result<Vec<ReplicaSet>, ServerFnError> {
    let response = kube_api_request(ApiType::Apps, "replicasets".to_string()).await?;
    let items = serde_json::from_str::<Response<ReplicaSet>>(&response)?
        .items
        .into_iter()
        .filter(|f| {
            f.metadata
                .namespace
                .contains(&namespace_name.clone().unwrap_or_default())
        })
        .collect();
    Ok(items)
}
