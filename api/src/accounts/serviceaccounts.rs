use domain::account::serviceaccount::ServiceAccount;
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::utils::{kube_api_request, ApiType};

#[server(GetServiceAccounts, "/api/accounts/serviceaccounts")]
pub async fn get_serviceaccounts(
    namespace_name: Option<String>,
) -> Result<Vec<ServiceAccount>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "serviceaccounts".to_string()).await?;
    let items = serde_json::from_str::<Response<ServiceAccount>>(&response)?
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
