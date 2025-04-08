use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_request;
#[allow(unused_imports)]
use crate::domain::storage::volume::{PersistentVolume, PersistentVolumesResponse};

#[server(GetVolumes, "/api/storage/volumes")]
pub async fn get_volumes() -> Result<Vec<PersistentVolume>, ServerFnError> {
    let response = kube_api_request("persistentvolumes".to_string()).await?;
    Ok(serde_json::from_str::<PersistentVolumesResponse>(&response)?.items)
}
