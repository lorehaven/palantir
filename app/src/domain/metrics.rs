use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Usage {
    pub cpu: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct NodeMetrics {
    pub timestamp: String,
    pub window: String,
    pub usage: Usage,
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
