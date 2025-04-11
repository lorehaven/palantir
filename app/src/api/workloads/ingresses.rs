use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_networking_request;
#[allow(unused_imports)]
use crate::domain::shared::response::Response;
use crate::domain::workload::ingress::Ingress;

#[server(GetIngresses, "/api/workloads/ingresses")]
pub async fn get_ingresses(
    namespace_name: Option<String>,
) -> Result<Vec<Ingress>, ServerFnError> {
    let response = kube_api_networking_request("ingresses".to_string()).await?;
    let items = serde_json::from_str::<Response<Ingress>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
