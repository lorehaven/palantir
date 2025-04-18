use domain::shared::response::Response;
use domain::storage::volume::PersistentVolume;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetVolumes, "/api/storage/volumes")]
pub async fn get_volumes() -> Result<Vec<PersistentVolume>, ServerFnError> {
    let response = resource_api::get("PersistentVolume".to_string(), None, None).await?;
    Ok(serde_json::from_str::<Response<PersistentVolume>>(&response)?.items)
}
