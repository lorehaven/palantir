use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::{Metadata, ResponseMetadata};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServiceAccountsResponse {
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub metadata: ResponseMetadata,
    pub items: Vec<ServiceAccount>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServiceAccount {
    pub metadata: Metadata,
    #[serde(default = "default_automount_service_account_token", rename = "automountServiceAccountToken")]
    pub automount_service_account_token: bool
}

const fn default_automount_service_account_token() -> bool {
    true
}
