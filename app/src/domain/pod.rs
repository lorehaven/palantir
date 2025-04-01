use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PodsResponse {
    pub items: Vec<Pod>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Pod {
    pub metadata: Metadata,
    pub spec: Spec,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Spec {
    #[serde(default)]
    pub volumes: Vec<Volume>,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Volume {
    #[serde(default)]
    name: String,
    #[serde(default)]
    #[serde(rename = "persistentVolumeClaim")]
    persistent_volume_claim: PersistentVolumeClaim,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PersistentVolumeClaim {
    #[serde(default)]
    #[serde(rename = "claimName")]
    claim_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Container {
    pub name: String,
    pub image: String,
    #[serde(default)]
    pub ports: Vec<Port>,
    #[serde(default)]
    pub env: Vec<Env>,
    pub resources: Resources,
    #[serde(rename = "terminationMessagePath")]
    pub termination_message_path: String,
    #[serde(rename = "terminationMessagePolicy")]
    pub termination_message_policy: String,
    #[serde(rename = "imagePullPolicy")]
    pub image_pull_policy: String,
}


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Port {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "containerPort")]
    pub container_port: i32,
    pub protocol: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Env {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Resources {
    #[serde(default)]
    pub limits: Resource,
    #[serde(default)]
    pub requests: Resource,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Resource {
    #[serde(default)]
    pub cpu: String,
    #[serde(default)]
    pub memory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Status {
    pub phase: String,
    pub conditions: Vec<Condition>,
    #[serde(rename = "hostIP")]
    pub host_ip: String,
    #[serde(default)]
    #[serde(rename = "podIP")]
    pub pod_ip: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "containerStatuses")]
    pub container_statuses: Vec<ContainerStatus>,
    #[serde(rename = "qosClass")]
    pub qos_class: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContainerStatus {
    pub name: String,
    pub ready: bool,
    #[serde(rename = "restartCount")]
    pub restart_count: i32,
    pub image: String,
    #[serde(rename = "imageID")]
    pub image_id: String,
    #[serde(default)]
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub started: bool,
}

