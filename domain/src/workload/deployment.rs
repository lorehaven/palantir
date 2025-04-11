use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::shared::metadata::Metadata;
use crate::shared::template::Template;
use crate::utils::time::time_until_now;
use crate::workload::{Workload, WorkloadModel};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Deployment {
    pub metadata: Metadata,
    pub spec: DeploymentSpec,
    pub status: DeploymentStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeploymentSpec {
    #[serde(rename = "progressDeadlineSeconds")]
    pub progress_deadline_seconds: i32,
    pub replicas: i32,
    #[serde(rename = "revisionHistoryLimit")]
    pub revision_history_limit: i32,
    pub selector: DeploymentSelector,
    pub strategy: DeploymentStrategy,
    pub template: Template,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeploymentSelector {
    #[serde(rename = "matchLabels")]
    pub match_labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeploymentStrategy {
    // rolling update
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeploymentStatus {
    #[serde(rename = "availableReplicas")]
    pub available_replicas: i32,
    pub conditions: Vec<Condition>,
    #[serde(rename = "observedGeneration")]
    pub observed_generation: i32,
    #[serde(rename = "readyReplicas")]
    pub ready_replicas: i32,
    pub replicas: i32,
    #[serde(rename = "updatedReplicas")]
    pub updated_replicas: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Condition {
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    pub message: String,
    pub reason: String,
    pub status: String,
    pub r#type: String,
}

impl Workload for Deployment {
    fn is_ready(&self) -> bool {
        self.status.conditions.iter().any(|c| c.r#type == "Available" && c.status == "True")
    }

    fn get_name(&self) -> String {
        self.metadata.name.clone()
    }

    fn to_model(&self) -> WorkloadModel {
        WorkloadModel {
            r#type: "Deployment".to_string(),
            name: self.metadata.name.clone(),
            namespace: self.metadata.namespace.clone(),
            age: time_until_now(&self.clone().metadata.creation_timestamp.unwrap_or_default()),
            pods: format!("{}/{}", self.status.available_replicas, self.status.replicas),
        }
    }
}
