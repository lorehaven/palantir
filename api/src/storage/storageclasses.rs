use domain::shared::response::Response;
use domain::storage::storageclass::StorageClass;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetStorageClasses, "/api/storage/storageclasses")]
pub async fn get_storageclasses() -> Result<Vec<StorageClass>, ServerFnError> {
    let response = resource_api::get("StorageClass".to_string(), None, None).await?;
    Ok(serde_json::from_str::<Response<StorageClass>>(&response)?.items)
}
