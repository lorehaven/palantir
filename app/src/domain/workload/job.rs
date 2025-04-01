use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::Metadata;
use crate::domain::workload::{Workload, WorkloadModel};
use crate::pages::utils::shared::time::time_until_now;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct JobsResponse {
    pub items: Vec<Job>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Job {
    pub metadata: Metadata,
    pub spec: JobSpec,
    pub status: JobStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct JobSpec {
    #[serde(rename = "backoffLimit")]
    pub backoff_limit: i32,
    #[serde(rename = "completionMode")]
    pub completion_mode: String,
    pub completions: i32,
    #[serde(rename = "manualSelector")]
    pub manual_selector: bool,
    pub parallelism: i32,
    #[serde(rename = "podReplacementPolicy")]
    pub pod_replacement_policy: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct JobStatus {
    #[serde(rename = "completionTime")]
    pub completion_time: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    pub conditions: Vec<Condition>,
    pub ready: i32,
    pub succeeded: i32,
    pub terminating: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Condition {
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    #[serde(rename = "lastProbeTime")]
    pub last_probe_time: String,
    pub message: String,
    pub reason: String,
    pub status: String,
    pub r#type: String,
}

impl Workload for Job {
    fn is_ready(&self) -> bool {
        self.status.conditions.iter().any(|c| c.r#type == "Complete" && c.status == "True")
    }

    fn get_name(&self) -> String {
        self.metadata.name.clone()
    }

    fn to_model(&self) -> WorkloadModel {
        WorkloadModel {
            r#type: "Job".to_string(),
            name: self.metadata.name.clone(),
            namespace: self.metadata.namespace.clone(),
            age: time_until_now(&self.metadata.creation_timestamp),
            pods: format!("{}/{}", self.status.ready, self.status.succeeded),
        }
    }
}
