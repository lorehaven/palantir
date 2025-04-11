use serde::{Deserialize, Serialize};

use crate::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StorageClass {
    pub metadata: Metadata,
    pub provisioner: String,
    #[serde(rename = "reclaimPolicy")]
    pub reclaim_policy: String,
    #[serde(rename = "volumeBindingMode")]
    pub volume_binding_mode: String,
}
