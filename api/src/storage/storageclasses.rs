use domain::shared::response::Response;
use domain::storage::storageclass::StorageClass;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::utils::{kube_api_request, ApiType};

#[server(GetStorageClasses, "/api/storage/storageclasses")]
pub async fn get_storageclasses() -> Result<Vec<StorageClass>, ServerFnError> {
    let response = kube_api_request(ApiType::Storage, "storageclasses".to_string()).await?;
    Ok(serde_json::from_str::<Response<StorageClass>>(&response)?.items)
}
