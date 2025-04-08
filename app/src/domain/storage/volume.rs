use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::{Metadata, ResponseMetadata};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PersistentVolumesResponse {
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub metadata: ResponseMetadata,
    pub items: Vec<PersistentVolume>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PersistentVolume {
    pub metadata: Metadata,
    pub spec: Spec,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Spec {
    #[serde(rename = "accessModes")]
    pub access_mode: Vec<String>,
    pub capacity: Capacity,
    #[serde(rename = "claimRef")]
    pub claim_ref: ClaimRef,
    #[serde(rename = "hostPath")]
    pub host_path: HostPath,
    #[serde(rename = "persistentVolumeReclaimPolicy")]
    pub persistent_volume_reclaim_policy: String,
    #[serde(rename = "volumeMode")]
    pub volume_mode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Status {
    #[serde(rename = "lastPhaseTransitionTime")]
    pub last_phase_transition_time: String,
    pub phase: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Capacity {
    pub storage: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClaimRef {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub name: String,
    pub namespace: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    pub uid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HostPath {
    path: String,
    r#type: String,
}
