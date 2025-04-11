use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::utils::{kube_api_request, ApiType};
#[allow(unused_imports)]
use domain::shared::response::Response;
use domain::workload::configmap::ConfigMap;

#[server(GetConfigs, "/api/workloads/configmaps")]
pub async fn get_configmaps(
    namespace_name: Option<String>,
) -> Result<Vec<ConfigMap>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "configmaps".to_string()).await?;
    let items = serde_json::from_str::<Response<ConfigMap>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
