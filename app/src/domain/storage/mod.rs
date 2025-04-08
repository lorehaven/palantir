use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::{Metadata, ResponseMetadata};

pub mod claim;
pub mod volume;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StorageClassesResponse {
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub metadata: ResponseMetadata,
    pub items: Vec<StorageClass>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StorageClass {
    pub metadata: Metadata,
    pub provisioner: String,
    #[serde(rename = "reclaimPolicy")]
    pub reclaim_policy: String,
    #[serde(rename = "volumeBindingMode")]
    pub volume_binding_mode: String,
}
