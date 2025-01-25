use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServicesResponse {
    pub items: Vec<Service>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Service {
    pub metadata: Metadata,
    pub spec: Spec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
    pub uid: String,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
    pub labels: HashMap<String, String>,
    pub annotations: Option<Annotations>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Annotations {
    #[serde(rename = "kubectl.kubernetes.io/last-applied-configuration")]
    pub last_applied_configuration: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spec {
    pub ports: Vec<Port>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    pub name: String,
    #[serde(rename = "nodePort")]
    pub port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceEntry {
    pub name: String,
    pub url: String,
    pub url_display: String,
    pub available: bool,
}
