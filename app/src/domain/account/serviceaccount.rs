use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServiceAccount {
    pub metadata: Metadata,
    #[serde(default = "default_automount_service_account_token", rename = "automountServiceAccountToken")]
    pub automount_service_account_token: bool
}

const fn default_automount_service_account_token() -> bool {
    true
}
