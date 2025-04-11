use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PersistentVolumeClaim {
    pub metadata: Metadata,
    pub spec: Spec,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Spec {
    #[serde(rename = "accessModes")]
    pub access_modes: Vec<String>,
    pub resources: Resources,
    #[serde(rename = "storageClassName")]
    pub storage_class_name: String,
    #[serde(rename = "volumeMode")]
    pub volume_mode: String,
    #[serde(rename = "volumeName")]
    pub volume_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Resources {
    pub requests: Requests,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Requests {
    pub storage: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Status {
    #[serde(rename = "accessModes")]
    pub access_modes: Vec<String>,
    pub capacity: Capacity,
    pub phase: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Capacity {
    pub storage: String,
}
