use domain::shared::response::Response;
use domain::storage::claim::PersistentVolumeClaim;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::utils::{kube_api_request, ApiType};

#[server(GetClaims, "/api/storage/claims")]
pub async fn get_claims(
    namespace_name: Option<String>,
) -> Result<Vec<PersistentVolumeClaim>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "persistentvolumeclaims".to_string()).await?;
    let items = serde_json::from_str::<Response<PersistentVolumeClaim>>(&response)?
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
