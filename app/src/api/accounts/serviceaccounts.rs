use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_request;
#[allow(unused_imports)]
use crate::domain::account::serviceaccount::{ServiceAccount, ServiceAccountsResponse};

#[server(GetServiceAccounts, "/api/accounts/serviceaccounts")]
pub async fn get_serviceaccounts(
    namespace_name: Option<String>,
) -> Result<Vec<ServiceAccount>, ServerFnError> {
    let response = kube_api_request("serviceaccounts".to_string()).await?;
    let items = serde_json::from_str::<ServiceAccountsResponse>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
