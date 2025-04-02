#[allow(unused_imports)]
use crate::api::utils::*;
use crate::domain::workload::Workload;

pub mod daemonsets;
pub mod deployments;
pub mod jobs;
pub mod pods;
pub mod services;

pub async fn get_workloads() -> Vec<Box<dyn Workload>> {
    let mut workloads = vec![];
    let daemonsets = daemonsets::get_daemonsets(None).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    let deployments = deployments::get_deployments(None).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    let jobs = jobs::get_jobs(None).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    workloads.extend(daemonsets);
    workloads.extend(deployments);
    workloads.extend(jobs);
    workloads
}

pub async fn get_workloads_by_namespace_name(namespace_name: String) -> Vec<Box<dyn Workload>> {
    let mut workloads = vec![];
    let daemonsets = daemonsets::get_daemonsets(Some(namespace_name.clone())).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    let deployments = deployments::get_deployments(Some(namespace_name.clone())).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    let jobs = jobs::get_jobs(Some(namespace_name)).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    workloads.extend(daemonsets);
    workloads.extend(deployments);
    workloads.extend(jobs);
    workloads
}
