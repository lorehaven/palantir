use domain::account::serviceaccount::ServiceAccount;
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetServiceAccounts, "/api/accounts/serviceaccounts")]
pub async fn get_serviceaccounts(
    namespace_name: Option<String>,
) -> Result<Vec<ServiceAccount>, ServerFnError> {
    let response = resource_api::get("ServiceAccount".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<ServiceAccount>>(&response)?.items)
}
