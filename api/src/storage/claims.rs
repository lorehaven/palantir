use domain::shared::response::Response;
use domain::storage::claim::PersistentVolumeClaim;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetClaims, "/api/storage/claims")]
pub async fn get_claims(
    namespace_name: Option<String>,
) -> Result<Vec<PersistentVolumeClaim>, ServerFnError> {
    let response =
        resource_api::get("PersistentVolumeClaim".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<PersistentVolumeClaim>>(&response)?.items)
}
