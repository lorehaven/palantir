use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_apps_request;
#[allow(unused_imports)]
use crate::domain::workload::replicaset::{ReplicaSet, ReplicaSetsResponse};

#[server(GetReplicaSets, "/api/workloads/replicasets")]
pub async fn get_replicasets(
    namespace_name: Option<String>,
) -> Result<Vec<ReplicaSet>, ServerFnError> {
    let response = kube_api_apps_request("replicasets".to_string()).await?;
    let items = serde_json::from_str::<ReplicaSetsResponse>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
