use leptos::prelude::ServerFnError;
use leptos::server;

const DEFAULT_TOKEN_PATH: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";

#[allow(dead_code)]
pub fn get_api_token() -> String {
    let token_path = std::env::var("KUBERNETES_TOKEN_PATH").unwrap_or_else(|_| DEFAULT_TOKEN_PATH.to_string());
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

#[server]
pub async fn kube_api_request(endpoint: String) -> Result<String, ServerFnError> {
    kube_api_request_internal("api/v1".to_string(), endpoint).await
}

#[server]
pub async fn kube_api_apps_request(endpoint: String) -> Result<String, ServerFnError> {
    kube_api_request_internal("apis/apps/v1".to_string(), endpoint).await
}

#[server]
pub async fn kube_api_batch_request(endpoint: String) -> Result<String, ServerFnError> {
    kube_api_request_internal("apis/batch/v1".to_string(), endpoint).await
}

#[server]
pub async fn kube_api_networking_request(endpoint: String) -> Result<String, ServerFnError> {
    kube_api_request_internal("apis/networking.k8s.io/v1".to_string(), endpoint).await
}

#[server]
async fn kube_api_request_internal(path: String, endpoint: String) -> Result<String, ServerFnError> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!("https://{server_host}:6443/{path}/{endpoint}"))
        .bearer_auth(get_api_token())
        .send()
        .await?;

    response.error_for_status_ref()?;
    Ok(response.text().await?)
}

#[server]
#[allow(clippy::unused_async)]
pub async fn get_url(payload: String) -> Result<String, ServerFnError> {
    let payload = serde_json::from_str::<serde_json::Value>(&payload)?;
    let resource_map = vec![
        ("apiextensions.k8s.io/v1", "customresourcedefinitions", "CustomResourceDefinition"),
        ("apps/v1", "daemonsets", "DaemonSet"),
        ("apps/v1", "deployments", "Deployment"),
        ("apps/v1", "replicasets", "ReplicaSet"),
        ("apps/v1", "statefulsets", "StatefulSet"),
        ("autoscaling/v2", "horizontalpodautoscalers", "HorizontalPodAutoscaler"),
        ("batch/v1", "cronjobs", "CronJob"),
        ("certificates.k8s.io/v1", "certificatesigningrequests", "CertificateSigningRequest"),
        ("networking.k8s.io/v1", "ingresses", "Ingress"),
        ("networking.k8s.io/v1", "networkpolicies", "NetworkPolicy"),
        ("policy/v1", "poddisruptionbudgets", "PodDisruptionBudget"),
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
        ("v1", "serviceaccounts", "ServiceAccount"),
        ("v1", "services", "Service"),
    ];

    let kind = payload.get("kind").unwrap().as_str().unwrap().to_string();
    let metadata = payload.get("metadata").unwrap();
    let namespace = metadata.get("namespace").unwrap().as_str().unwrap().to_string();
    if let Some((u, k, _)) = resource_map.iter().find(|(_, _, k)| k == &kind) {
        let prefix = if u.starts_with("v1") { "api" } else { "apis" };
        Ok(format!("{prefix}/{u}/namespaces/{namespace}/{k}"))
    } else {
        Err(ServerFnError::ServerError("invalid resource - cannot build url".to_string()))
    }
}
