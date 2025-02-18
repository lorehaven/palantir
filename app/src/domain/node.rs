use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodesResponse {
    pub items: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub metadata: Metadata,
    pub spec: Spec,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub uid: String,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
    pub labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spec {
    #[serde(rename = "podCIDR")]
    pub pod_cidr: String,
    #[serde(rename = "podCIDRs")]
    pub pod_cidrs: Vec<String>,
    #[serde(rename = "providerID")]
    pub provider_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub addresses: Vec<Address>,
    pub allocatable: Allocatable,
    pub capacity: Capacity,
    pub conditions: Vec<Condition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub r#type: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Allocatable {
    pub cpu: String,
    #[serde(rename = "ephemeral-storage")]
    pub ephemeral_storage: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Capacity {
    pub cpu: String,
    #[serde(rename = "ephemeral-storage")]
    pub ephemeral_storage: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
    pub r#type: String,
    pub status: String,
    #[serde(rename = "lastHeartbeatTime")]
    pub last_heartbeat_time: String,
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    pub reason: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NodeType {
    ControlPlane,
    Master,
    Worker,
}

impl NodeType {
    pub fn from_node(node: &Node) -> Self {
        if node.metadata.labels.get("node-role.kubernetes.io/control-plane") == Some(&"true".to_string()) {
            NodeType::ControlPlane
        } else if node.metadata.labels.get("node-role.kubernetes.io/master") == Some(&"true".to_string()) {
            NodeType::Master
        } else {
            NodeType::Worker
        }
    }
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::ControlPlane => write!(f, "Control Plane"),
            NodeType::Master => write!(f, "Master"),
            NodeType::Worker => write!(f, "Worker"),
        }
    }
}
