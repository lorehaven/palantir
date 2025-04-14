use domain::shared::response::Response;
use domain::workload::ingress::Ingress;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::utils::{kube_api_request, ApiType};

#[server(GetIngresses, "/api/workloads/ingresses")]
pub async fn get_ingresses(namespace_name: Option<String>) -> Result<Vec<Ingress>, ServerFnError> {
    let response = kube_api_request(ApiType::Networking, "ingresses".to_string()).await?;
    let items = serde_json::from_str::<Response<Ingress>>(&response)?
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
