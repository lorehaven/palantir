use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NamespacesResponse {
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub metadata: ResponseMetadata,
    pub items: Vec<Namespace>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResponseMetadata {
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Namespace {
    pub metadata: Metadata,
    pub spec: Spec,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Metadata {
    pub name: String,
    pub uid: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Spec {
    pub finalizers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Status {
    pub phase: String,
}
