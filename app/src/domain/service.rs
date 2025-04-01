use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::Metadata;

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
