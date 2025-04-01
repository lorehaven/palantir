use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::*;
use crate::domain::workload::{*, deployment::*, job::*, daemonset::*};

pub async fn get_workloads() -> Vec<Box<dyn Workload>> {
    let mut workloads = vec![];
    let daemonsets = get_daemonsets(None).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    let deployments = get_deployments(None).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    let jobs = get_jobs(None).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    workloads.extend(daemonsets);
    workloads.extend(deployments);
    workloads.extend(jobs);
    workloads
}

pub async fn get_workloads_by_namespace_name(namespace_name: String) -> Vec<Box<dyn Workload>> {
    let mut workloads = vec![];
    let daemonsets = get_daemonsets(Some(namespace_name.clone())).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    let deployments = get_deployments(Some(namespace_name.clone())).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    let jobs = get_jobs(Some(namespace_name)).await.unwrap_or_default()
        .into_iter()
        .map(|d| Box::new(d) as Box<dyn Workload>);
    workloads.extend(daemonsets);
    workloads.extend(deployments);
    workloads.extend(jobs);
    workloads
}

#[server(GetDaemonsets, "/api/workloads/daemonsets")]
pub async fn get_daemonsets(
    namespace_name: Option<String>,
) -> Result<Vec<Daemonset>, ServerFnError> {
    let response = kube_api_apps_request("daemonsets".to_string()).await?;
    let items = serde_json::from_str::<DaemonsetsResponse>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}

#[server(GetDeployments, "/api/workloads/deployments")]
pub async fn get_deployments(
    namespace_name: Option<String>,
) -> Result<Vec<Deployment>, ServerFnError> {
    let response = kube_api_apps_request("deployments".to_string()).await?;
    let items = serde_json::from_str::<DeploymentsResponse>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}

#[server(GetJobs, "/api/workloads/jobs")]
pub async fn get_jobs(
    namespace_name: Option<String>,
) -> Result<Vec<Job>, ServerFnError> {
    let response = kube_api_batch_request("jobs".to_string()).await?;
    let items = serde_json::from_str::<JobsResponse>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
