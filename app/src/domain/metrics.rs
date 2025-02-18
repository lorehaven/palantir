use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Usage {
    pub cpu: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct NodeMetrics {
    pub metadata: NodeMetricsMetadata,
    pub timestamp: String,
    pub window: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct NodeMetricsMetadata {
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
    pub labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ContainerMetrics {
    pub name: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct PodMetrics {
    pub timestamp: String,
    pub window: String,
    pub containers: Vec<ContainerMetrics>,
}

impl NodeMetrics {
    pub fn get_node_name(&self) -> String {
        self.metadata.labels.get("kubernetes.io/hostname").unwrap().to_string()
    }
}
