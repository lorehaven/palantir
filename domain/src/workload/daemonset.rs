use serde::{Deserialize, Serialize};

use crate::shared::metadata::Metadata;
use crate::shared::template::Template;
use crate::utils::time::time_until_now;
use crate::workload::{Workload, WorkloadModel};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DaemonSet {
    pub metadata: Metadata,
    pub spec: DaemonSetSpec,
    pub status: DaemonSetStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DaemonSetSpec {
    #[serde(rename = "revisionHistoryLimit")]
    pub revision_history_limit: i32,
    // selector
    pub template: Template,
    // update strategy
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DaemonSetStatus {
    #[serde(rename = "currentNumberScheduled")]
    pub current_number_scheduled: i32,
    #[serde(rename = "desiredNumberScheduled")]
    pub desired_number_scheduled: i32,
    #[serde(rename = "numberAvailable")]
    pub number_available: i32,
    #[serde(rename = "numberMisscheduled")]
    pub number_misscheduled: i32,
    #[serde(rename = "numberReady")]
    pub number_ready: i32,
    #[serde(rename = "observedGeneration")]
    pub observed_generation: i32,
    #[serde(rename = "updatedNumberScheduled")]
    pub updated_number_scheduled: i32,
}

impl Workload for DaemonSet {
    fn is_ready(&self) -> bool {
        self.status.current_number_scheduled == self.status.number_ready
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
            pods: format!("{}/{}", self.status.number_ready, self.status.number_available),
        }
    }
}
