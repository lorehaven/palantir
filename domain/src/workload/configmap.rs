use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConfigMap {
    #[serde(default)]
    pub data: HashMap<String, String>,
    pub metadata: Metadata,
}
