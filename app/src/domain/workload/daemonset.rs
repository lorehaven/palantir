use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::Metadata;
use crate::domain::workload::{Workload, WorkloadModel};
use crate::pages::utils::shared::time::time_until_now;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DaemonsetsResponse {
    pub items: Vec<Daemonset>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Daemonset {
    pub metadata: Metadata,
    pub spec: DaemonsetSpec,
    pub status: DaemonsetStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DaemonsetSpec {
    #[serde(rename = "revisionHistoryLimit")]
    pub revision_history_limit: i32,
    // selector
    // template
    // update strategy
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DaemonsetStatus {
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

impl Workload for Daemonset {
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
            age: time_until_now(&self.metadata.creation_timestamp),
            pods: format!("{}/{}", self.status.number_ready, self.status.number_available),
        }
    }
}
