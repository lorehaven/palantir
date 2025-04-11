use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::{ApiType, kube_api_request};
#[allow(unused_imports)]
use crate::domain::shared::response::Response;
use crate::domain::storage::storageclass::StorageClass;

#[server(GetStorageClasses, "/api/storage/storageclasses")]
pub async fn get_storageclasses() -> Result<Vec<StorageClass>, ServerFnError> {
    let response = kube_api_request(ApiType::Storage, "storageclasses".to_string()).await?;
    Ok(serde_json::from_str::<Response<StorageClass>>(&response)?.items)
}
