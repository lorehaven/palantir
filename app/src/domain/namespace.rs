use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::Metadata;

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
pub struct Spec {
    pub finalizers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Status {
    pub phase: String,
}
