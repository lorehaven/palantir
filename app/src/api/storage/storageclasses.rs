use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_storage_request;
#[allow(unused_imports)]
use crate::domain::shared::response::Response;
use crate::domain::storage::storageclass::StorageClass;

#[server(GetStorageClasses, "/api/storage/storageclasses")]
pub async fn get_storageclasses() -> Result<Vec<StorageClass>, ServerFnError> {
    let response = kube_api_storage_request("storageclasses".to_string()).await?;
    Ok(serde_json::from_str::<Response<StorageClass>>(&response)?.items)
}
