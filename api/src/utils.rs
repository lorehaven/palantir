use quench_cache::CacheStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ApiMode {
    Get,
    Delete,
    Post,
    Put,
}

/// How long a cached token stays valid. Comfortably under a projected K8s
/// `ServiceAccount` token's typical rotation window (kubelet refreshes the
/// file well before expiry), so a cache hit is never staler than the
/// kubelet's own refresh cadence would already allow.
const TOKEN_CACHE_TTL_SECS: u64 = 60;
const TOKEN_CACHE_KEY: &str = "k8s-token";

fn read_token_file() -> String {
    std::fs::read_to_string(crate::config::kubernetes_token_path())
        .expect("token file is missing.")
        .trim()
        .to_string()
}

/// Cached read of the K8s `ServiceAccount` token.
pub async fn get_api_token(store: &CacheStore) -> String {
    if let Ok(Some(cached)) = store.get(TOKEN_CACHE_KEY).await {
        if let Some(token) = cached.as_str() {
            return token.to_string();
        }
    }

    let token = read_token_file();
    let _ = store
        .set(
            TOKEN_CACHE_KEY,
            serde_json::Value::String(token.clone()),
            Some(TOKEN_CACHE_TTL_SECS),
        )
        .await;
    token
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

pub fn get_url(
    kind: &str,
    namespace: Option<String>,
    resource_name: Option<String>,
) -> anyhow::Result<String> {
    let resource_map = get_resource_map();

    let Some((u, k, _)) = resource_map.iter().find(|(_, _, rk)| *rk == kind) else {
        return Err(anyhow::anyhow!("invalid resource - cannot build url"));
    };
    let prefix = if u.starts_with("v1") { "api" } else { "apis" };

    Ok(match (namespace, resource_name) {
        (None, None) => format!("{prefix}/{u}/{k}"),
        (None, Some(resource_name)) => format!("{prefix}/{u}/{k}/{resource_name}"),
        (Some(ns), None) => format!("{prefix}/{u}/namespaces/{ns}/{k}"),
        (Some(ns), Some(resource_name)) => {
            format!("{prefix}/{u}/namespaces/{ns}/{k}/{resource_name}")
        }
    })
}
