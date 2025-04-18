use domain::account::secret::Secret;
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetSecrets, "/api/accounts/secrets")]
pub async fn get_secrets(namespace_name: Option<String>) -> Result<Vec<Secret>, ServerFnError> {
    let response = resource_api::get("Secret".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Secret>>(&response)?.items)
}
