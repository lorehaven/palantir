use std::fmt::Debug;
use serde::{Deserialize, Serialize};

pub mod daemonset;
pub mod deployment;
pub mod ingress;
pub mod job;
pub mod replicaset;
pub mod service;

pub trait Workload: Debug + Sync + Send {
    fn is_ready(&self) -> bool;
    fn get_name(&self) -> String;
    fn to_model(&self) -> WorkloadModel;
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WorkloadModel {
    pub r#type: String,
    pub name: String,
    pub namespace: String,
    pub age: String,
    pub pods: String,
}
