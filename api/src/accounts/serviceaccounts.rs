use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::utils::{kube_api_request, ApiType};
use domain::account::serviceaccount::ServiceAccount;
#[allow(unused_imports)]
use domain::shared::response::Response;

#[server(GetServiceAccounts, "/api/accounts/serviceaccounts")]
pub async fn get_serviceaccounts(
    namespace_name: Option<String>,
) -> Result<Vec<ServiceAccount>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "serviceaccounts".to_string()).await?;
    let items = serde_json::from_str::<Response<ServiceAccount>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
