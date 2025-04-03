use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Template {
    pub metadata: Metadata,
    pub spec: Spec,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Spec {
    pub containers: Vec<Container>,
    #[serde(rename = "dnsPolicy")]
    pub dns_policy: String,
    #[serde(rename = "restartPolicy")]
    pub restart_policy: String,
    #[serde(rename = "schedulerName")]
    pub scheduler_name: String,
    #[serde(rename = "securityContext")]
    pub security_context: SecurityContext,
    #[serde(rename = "serviceAccount")]
    pub service_account: String,
    #[serde(rename = "serviceAccountName")]
    pub service_account_name: String,
    #[serde(rename = "terminationGracePeriodSeconds")]
    pub termination_grace_period_seconds: i32,
    #[serde(default)]
    pub volumes: Vec<Volume>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SecurityContext {
    #[serde(default, rename = "serviceAccountName")]
    pub fs_group: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Container {
    #[serde(default)]
    pub env: Vec<Env>,
    pub image: String,
    pub name: String,
    #[serde(default)]
    pub ports: Vec<Port>,
    #[serde(default)]
    pub resources: Resources,
    #[serde(rename = "terminationMessagePath")]
    pub termination_message_path: String,
    #[serde(rename = "terminationMessagePolicy")]
    pub termination_message_policy: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Env {
    pub name: String,
    #[serde(default)]
    pub value: String,
    #[serde(default, rename = "valueFrom")]
    pub value_from: ValueFrom,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ValueFrom {
    #[serde(default, rename = "fieldRef")]
    pub field_ref: FieldRef,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FieldRef {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Port {
    #[serde(default, rename = "containerPort")]
    pub container_port: i32,
    #[serde(default)]
    pub name: String,
    pub protocol: String,
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
pub struct Volume {
    #[serde(default, rename = "hostPath")]
    pub host_path: HostPath,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HostPath {
    pub path: String,
    pub r#type: String,
}
