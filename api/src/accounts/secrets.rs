use domain::account::secret::Secret;
#[allow(unused_imports)]
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::utils::{kube_api_request, ApiType};

#[server(GetSecrets, "/api/accounts/secrets")]
pub async fn get_secrets(namespace_name: Option<String>) -> Result<Vec<Secret>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "secrets".to_string()).await?;
    let items = serde_json::from_str::<Response<Secret>>(&response)?
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
