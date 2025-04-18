use domain::cluster::pod::Pod;
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetPods, "/api/pods")]
pub async fn get_pods(
    namespace_name: Option<String>,
    node_name: Option<String>,
) -> Result<Vec<Pod>, ServerFnError> {
    let response = resource_api::get("Pod".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Pod>>(&response)?
        .items
        .into_iter()
        .filter(|f| {
            f.spec
                .node_name
                .contains(&node_name.clone().unwrap_or_default())
        })
        .collect())
}
