use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct NodeMetrics {
    pub metadata: NodeMetricsMetadata,
    pub timestamp: String,
    pub window: String,
    pub usage: Usage,
}

impl NodeMetrics {
    pub fn get_node_name(&self) -> String {
        self.metadata.labels.get("kubernetes.io/hostname").unwrap().to_string()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct NodeMetricsMetadata {
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
    pub labels: HashMap<String, String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct PodMetrics {
    pub metadata: PodMetricsMetadata,
    pub timestamp: String,
    pub window: String,
    pub containers: Vec<ContainerMetrics>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct PodMetricsMetadata {
    pub name: String,
    pub namespace: String,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
    pub labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ContainerMetrics {
    #[serde(default)]
    pub name: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Usage {
    pub cpu: String,
    pub memory: String,
}
