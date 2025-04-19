use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ApiMode {
    Get,
    Delete,
    Post,
    Put,
}

const DEFAULT_TOKEN_PATH: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";

pub fn get_api_token() -> String {
    let token_path =
        std::env::var("KUBERNETES_TOKEN_PATH").unwrap_or_else(|_| DEFAULT_TOKEN_PATH.to_string());
    std::fs::read_to_string(token_path)
        .expect("token file is missing.")
        .trim()
        .to_string()
}

#[server]
#[allow(clippy::unused_async)]
pub async fn get_api_token_wasm() -> Result<String, ServerFnError> {
    Ok(get_api_token())
}

fn get_resource_map() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "apiextensions.k8s.io/v1",
            "customresourcedefinitions",
            "CustomResourceDefinition",
        ),
        ("apps/v1", "daemonsets", "DaemonSet"),
        ("apps/v1", "deployments", "Deployment"),
        ("apps/v1", "replicasets", "ReplicaSet"),
        ("apps/v1", "statefulsets", "StatefulSet"),
        (
            "autoscaling/v2",
            "horizontalpodautoscalers",
            "HorizontalPodAutoscaler",
        ),
        ("batch/v1", "cronjobs", "CronJob"),
        ("batch/v1", "jobs", "Job"),
        (
            "certificates.k8s.io/v1",
            "certificatesigningrequests",
            "CertificateSigningRequest",
        ),
        ("networking.k8s.io/v1", "ingresses", "Ingress"),
        ("networking.k8s.io/v1", "networkpolicies", "NetworkPolicy"),
        ("policy/v1", "poddisruptionbudgets", "PodDisruptionBudget"),
        (
            "rbac.authorization.k8s.io/v1",
            "clusterrolebindings",
            "ClusterRoleBinding",
        ),
        (
            "rbac.authorization.k8s.io/v1",
            "rolebindings",
            "RoleBinding",
        ),
        (
            "rbac.authorization.k8s.io/v1",
            "clusterroles",
            "ClusterRole",
        ),
        ("rbac.authorization.k8s.io/v1", "roles", "Role"),
        ("scheduling.k8s.io/v1", "priorityclasses", "PriorityClass"),
        ("storage.k8s.io/v1", "storageclasses", "StorageClass"),
        ("v1", "componentstatuses", "ComponentStatus"),
        ("v1", "configmaps", "ConfigMap"),
        ("v1", "endpoints", "Endpoints"),
        ("v1", "events", "Event"),
        ("v1", "limitranges", "LimitRange"),
        ("v1", "namespaces", "Namespace"),
        ("v1", "nodes", "Node"),
        ("v1", "persistentvolumeclaims", "PersistentVolumeClaim"),
        ("v1", "persistentvolumes", "PersistentVolume"),
        ("v1", "pods", "Pod"),
        ("v1", "replicationcontrollers", "ReplicationController"),
        ("v1", "resourcequotas", "ResourceQuota"),
        ("v1", "secrets", "Secret"),
        ("v1", "serviceaccounts", "ServiceAccount"),
        ("v1", "services", "Service"),
    ]
}

#[server]
#[allow(clippy::unused_async)]
pub async fn get_url(
    kind: String,
    namespace: Option<String>,
    resource_name: Option<String>,
) -> Result<String, ServerFnError> {
    let resource_map = get_resource_map();

    namespace.map_or_else(
        || {
            if let Some((u, k, _)) = resource_map.iter().find(|(_, _, k)| k == &kind) {
                let prefix = if u.starts_with("v1") { "api" } else { "apis" };
                resource_name.clone().map_or_else(
                    || Ok(format!("{prefix}/{u}/{k}")),
                    |resource_name| Ok(format!("{prefix}/{u}/{k}/{resource_name}")),
                )
            } else {
                Err(ServerFnError::ServerError(
                    "invalid resource - cannot build url".to_string(),
                ))
            }
        },
        |ns| {
            if let Some((u, k, _)) = resource_map.iter().find(|(_, _, k)| k == &kind) {
                let prefix = if u.starts_with("v1") { "api" } else { "apis" };
                resource_name.clone().map_or_else(
                    || Ok(format!("{prefix}/{u}/namespaces/{ns}/{k}")),
                    |resource_name| Ok(format!("{prefix}/{u}/namespaces/{ns}/{k}/{resource_name}")),
                )
            } else {
                Err(ServerFnError::ServerError(
                    "invalid resource - cannot build url".to_string(),
                ))
            }
        },
    )
}
