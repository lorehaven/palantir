use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PodsResponse {
    pub items: Vec<Pod>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pod {
    pub metadata: Metadata,
    pub spec: Spec,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spec {
    // volumes
    pub containers: Vec<Container>,
    #[serde(rename = "restartPolicy")]
    pub restart_policy: String,
    #[serde(rename = "terminationGracePeriodSeconds")]
    pub termination_grace_period_seconds: i32,
    #[serde(rename = "dnsPolicy")]
    pub dns_policy: String,
    #[serde(rename = "serviceAccountName")]
    pub service_account_name: String,
    #[serde(rename = "serviceAccount")]
    pub service_account: String,
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(default)]
    #[serde(rename = "hostNetwork")]
    pub host_network: Option<bool>,
    // security context
    pub priority: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub name: String,
    #[serde(rename = "generateName")]
    pub generate_name: String,
    pub namespace: String,
    pub uid: String,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
    pub labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Container {
    pub name: String,
    pub image: String,
    #[serde(rename = "imagePullPolicy")]
    pub image_pull_policy: String,
    // ports
    // env
    pub resources: Resources,
    // ...
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Resources {
    #[serde(default)]
    pub limits: Resource,
    #[serde(default)]
    pub requests: Resource,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Resource {
    #[serde(default)]
    pub cpu: String,
    #[serde(default)]
    pub memory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub phase: String,
    #[serde(rename = "hostIP")]
    pub host_ip: String,
    #[serde(default)]
    #[serde(rename = "podIP")]
    pub pod_ip: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "qosClass")]
    pub qos_class: String,
    pub conditions: Vec<Condition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
    pub r#type: String,
    pub status: String,
    #[serde(default)]
    #[serde(rename = "lastProbeTime")]
    pub last_probe_time: Option<String>,
    #[serde(default)]
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
}
