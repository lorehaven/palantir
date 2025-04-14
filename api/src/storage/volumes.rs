use domain::shared::response::Response;
use domain::storage::volume::PersistentVolume;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::utils::{kube_api_request, ApiType};

#[server(GetVolumes, "/api/storage/volumes")]
pub async fn get_volumes() -> Result<Vec<PersistentVolume>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "persistentvolumes".to_string()).await?;
    Ok(serde_json::from_str::<Response<PersistentVolume>>(&response)?.items)
}
