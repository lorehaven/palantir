use serde::{Deserialize, Serialize};

use crate::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Node {
    pub metadata: Metadata,
    pub spec: Spec,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Spec {
    #[serde(rename = "podCIDR")]
    pub pod_cidr: String,
    #[serde(rename = "podCIDRs")]
    pub pod_cidrs: Vec<String>,
    #[serde(rename = "providerID")]
    pub provider_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Status {
    pub addresses: Vec<Address>,
    pub allocatable: Allocatable,
    pub capacity: Capacity,
    pub conditions: Vec<Condition>,
    #[serde(rename = "nodeInfo")]
    pub node_info: NodeInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Address {
    pub r#type: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Allocatable {
    pub cpu: String,
    #[serde(rename = "ephemeral-storage")]
    pub ephemeral_storage: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Capacity {
    pub cpu: String,
    #[serde(rename = "ephemeral-storage")]
    pub ephemeral_storage: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NodeInfo {
    #[serde(rename = "machineID")]
    pub machine_id: String,
    #[serde(rename = "systemUUID")]
    pub system_uuid: String,
    #[serde(rename = "bootID")]
    pub boot_id: String,
    #[serde(rename = "kernelVersion")]
    pub kernel_version: String,
    #[serde(rename = "osImage")]
    pub os_image: String,
    #[serde(rename = "containerRuntimeVersion")]
    pub container_runtime_version: String,
    #[serde(rename = "kubeletVersion")]
    pub kubelet_version: String,
    #[serde(rename = "kubeProxyVersion")]
    pub kube_proxy_version: String,
    #[serde(rename = "operatingSystem")]
    pub operating_system: String,
    pub architecture: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum NodeType {
    #[default]
    ControlPlane,
    Master,
    Worker,
}

impl NodeType {
    pub fn from_node(node: &Node) -> Self {
        if node
            .metadata
            .labels
            .get("node-role.kubernetes.io/control-plane")
            == Some(&"true".to_string())
        {
            Self::ControlPlane
        } else if node.metadata.labels.get("node-role.kubernetes.io/master")
            == Some(&"true".to_string())
        {
            Self::Master
        } else {
            Self::Worker
        }
    }
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ControlPlane => write!(f, "Control Plane"),
            Self::Master => write!(f, "Master"),
            Self::Worker => write!(f, "Worker"),
        }
    }
}
