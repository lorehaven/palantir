use serde::{Deserialize, Serialize};

use crate::shared::metadata::Metadata;
use crate::shared::template::Template;
use crate::utils::time::time_until_now;
use crate::workload::{Workload, WorkloadModel};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplicaSet {
    pub metadata: Metadata,
    pub spec: ReplicaSetSpec,
    pub status: ReplicaSetStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplicaSetSpec {
    #[serde(default)]
    pub replicas: i32,
    // selector
    pub template: Template,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplicaSetStatus {
    #[serde(default, rename = "availableReplicas")]
    pub available_replicas: i32,
    #[serde(default, rename = "fullyLabeledReplicas")]
    pub fully_labeled_replicas: i32,
    #[serde(default, rename = "observedGeneration")]
    pub observed_generation: i32,
    #[serde(default, rename = "readyReplicas")]
    pub ready_replicas: i32,
    #[serde(default)]
    pub replicas: i32,
}

impl Workload for ReplicaSet {
    fn is_ready(&self) -> bool {
        self.status.ready_replicas == self.status.replicas
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
            pods: format!("{}/{}", self.status.ready_replicas, self.status.replicas),
        }
    }
}
