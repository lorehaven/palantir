use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Metadata {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub uid: String,
    #[serde(default, rename = "generateName")]
    pub generate_name: String,
    #[serde(default, rename = "resourceVersion")]
    pub resource_version: String,
    #[serde(default, rename = "creationTimestamp")]
    pub creation_timestamp: String,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
}
