use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::{ApiType, kube_api_request};
#[allow(unused_imports)]
use crate::domain::shared::response::Response;
use crate::domain::account::secret::Secret;

#[server(GetSecrets, "/api/accounts/secrets")]
pub async fn get_secrets(
    namespace_name: Option<String>,
) -> Result<Vec<Secret>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "secrets".to_string()).await?;
    let items = serde_json::from_str::<Response<Secret>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
