use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConfigMapsResponse {
    pub items: Vec<ConfigMap>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConfigMap {
    #[serde(default)]
    pub data: HashMap<String, String>,
    pub metadata: Metadata,
}
