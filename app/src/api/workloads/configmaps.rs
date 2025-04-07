use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_request;
#[allow(unused_imports)]
use crate::domain::workload::configmap::{ConfigMap, ConfigMapsResponse};

#[server(GetConfigs, "/api/workloads/configmaps")]
pub async fn get_configmaps(
    namespace_name: Option<String>,
) -> Result<Vec<ConfigMap>, ServerFnError> {
    let response = kube_api_request("configmaps".to_string()).await?;
    let items = serde_json::from_str::<ConfigMapsResponse>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
