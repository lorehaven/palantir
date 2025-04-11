use domain::workload::Workload;

#[allow(unused_imports)]
use crate::utils::*;

pub mod configmaps;
pub mod daemonsets;
pub mod deployments;
pub mod ingresses;
pub mod jobs;
pub mod pods;
pub mod replicasets;
pub mod services;

pub async fn get_workloads(namespace_name: Option<String>) -> Vec<Box<dyn Workload>> {
    let mut workloads = vec![];
    let daemonsets = daemonsets::get_daemonsets(namespace_name.clone())
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    let deployments = deployments::get_deployments(namespace_name.clone())
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    let jobs = jobs::get_jobs(namespace_name)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    workloads.extend(daemonsets);
    workloads.extend(deployments);
    workloads.extend(jobs);
    workloads
}
