use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scale {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub metadata: Metadata,
    pub spec: Spec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub namespace: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spec {
    pub replicas: i64,
}

impl Scale {
    pub fn new(namespace: &str, name: &str, replicas: i64) -> Self {
        Self {
            api_version: "autoscaling/v1".to_string(),
            kind: "Scale".to_string(),
            metadata: Metadata {
                namespace: namespace.to_string(),
                name: name.to_string(),
            },
            spec: Spec { replicas },
        }
    }
}
