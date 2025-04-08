use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_storage_request;
#[allow(unused_imports)]
use crate::domain::storage::{StorageClass, StorageClassesResponse};

pub mod claims;
pub mod volumes;

#[server(GetStorageClasses, "/api/storage/storageclasses")]
pub async fn get_storageclasses() -> Result<Vec<StorageClass>, ServerFnError> {
    let response = kube_api_storage_request("storageclasses".to_string()).await?;
    Ok(serde_json::from_str::<StorageClassesResponse>(&response)?.items)
}
