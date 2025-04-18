use domain::shared::response::Response;
use domain::workload::configmap::ConfigMap;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetConfigs, "/api/workloads/configmaps")]
pub async fn get_configmaps(
    namespace_name: Option<String>,
) -> Result<Vec<ConfigMap>, ServerFnError> {
    let response = resource_api::get("ConfigMap".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<ConfigMap>>(&response)?.items)
}
