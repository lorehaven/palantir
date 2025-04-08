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
    pub creation_timestamp: Option<String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub generation: i32,
    #[serde(default, rename = "ownerReferences")]
    pub owner_references: Vec<OwnerReference>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResponseMetadata {
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OwnerReference {
    #[serde(default, rename = "apiVersion")]
    pub api_version: String,
    #[serde(default, rename = "blockOwnerDeletion")]
    pub block_owner_deletion: bool,
    #[serde(default)]
    pub controller: bool,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub uid: String,
}
