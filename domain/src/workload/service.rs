use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Service {
    pub metadata: Metadata,
    pub spec: Spec,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Spec {
    pub ports: Vec<Port>,
    #[serde(default)]
    pub selector: HashMap<String, String>,
    #[serde(rename = "clusterIP")]
    pub cluster_ip: String,
    #[serde(rename = "clusterIPs")]
    pub cluster_ips: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: String,
    #[serde(rename = "ipFamilies")]
    pub ip_families: Vec<String>,
    #[serde(rename = "ipFamilyPolicy")]
    pub ip_family_policy: String,
    #[serde(rename = "internalTrafficPolicy")]
    pub internal_traffic_policy: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Port {
    pub name: String,
    #[serde(default)]
    pub protocol: String,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default, rename = "targetPort")]
    pub target_port: Option<PortValue>,
    #[serde(default, rename = "nodePort")]
    pub node_port: Option<PortValue>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum PortValue {
    Number(u16),
    String(String),
}

impl Display for PortValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Status {
    #[serde(rename = "loadBalancer")]
    pub load_balancer: LoadBalancer,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoadBalancer {
    #[serde(default)]
    pub ingress: Option<Vec<Ingress>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Ingress {
    pub ip: String,
    #[serde(rename = "ipMode")]
    pub ip_mode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServiceEntry {
    pub name: String,
    pub url: String,
    pub url_display: String,
    pub available: bool,
}
