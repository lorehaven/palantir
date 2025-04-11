use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::{Metadata, ResponseMetadata};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SecretsResponse {
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub metadata: ResponseMetadata,
    pub items: Vec<Secret>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Secret {
    #[serde(default)]
    pub data: HashMap<String, String>,
    pub metadata: Metadata,
    pub r#type: String,
}
