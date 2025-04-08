use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::workloads::pods::get_pods;
#[allow(unused_imports)]
use crate::api::utils::kube_api_request;
use crate::domain::cluster::pod::Pod;
use crate::domain::workload::service::{ServiceEntry, ServicesResponse, Service};

const NAME_LABEL: &str = "app.kubernetes.io/name";

#[server(GetServiceEntries, "/api/services/entries")]
pub async fn get_service_entries() -> Result<Vec<ServiceEntry>, ServerFnError> {
    let services = kube_api_request("services".to_string()).await?;
    Ok(parse_entries_response(&services, &get_pods().await?).await.unwrap_or_default())
}

#[allow(dead_code)]
async fn parse_entries_response(response: &str, pods: &[Pod]) -> Result<Vec<ServiceEntry>, Box<dyn std::error::Error>> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let server_dns_name = std::env::var("SERVER_DNS_NAME").unwrap_or_else(|_| "ossiriand.arda".to_string());

    let mut services = serde_json::from_str::<ServicesResponse>(response)?
        .items
        .into_iter()
        .filter(|service| service.metadata.namespace != "default")
        .filter(|service| service.spec.r#type == "NodePort")
        .flat_map(|s| {
            let server_host = server_host.clone();
            let server_dns_name = server_dns_name.clone();
            let selector = get_service_selector(&s);
            s.spec.ports.into_iter().map(move |p| ServiceEntry {
                name: format_service_name(&p.name),
                url: format!("http://{server_host}:{}", p.port.unwrap()),
                url_display: format!("{server_dns_name}:{}", p.port.unwrap()),
                available: is_pod_available(get_pod_by_label(pods, &selector)),
            })
        })
        .collect::<Vec<ServiceEntry>>();

    let additional_services_json = std::env::var("ADDITIONAL_SERVICES").unwrap_or_else(|_| "[]".to_string());
    let mut additional_entries = serde_json::from_str::<Vec<ServiceEntry>>(&additional_services_json)?;
    for s in &mut additional_entries.iter_mut() {
        if s.available { continue; }
        s.available = reqwest::get(&s.url).await.is_ok_and(|r| r.status().is_success());
    }

    services.extend(additional_entries);
    services.sort_by_key(|e| e.url_display.clone());
    Ok(services
        .into_iter()
        .filter(|s| s.name.to_lowercase().contains("web ui"))
        .map(|mut s| { s.name = s.name.replace(" Web UI", ""); s })
        .collect())
}

#[allow(dead_code)]
fn format_service_name(service_name: &str) -> String {
    service_name
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            chars.next()
                .map(|first| format!("{}{}", first.to_uppercase(), chars.as_str()))
                .map(|word| if word.to_lowercase().ends_with("ui") { format!("{}UI", &word[..word.len() - 2]) } else { word })
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[allow(dead_code)]
fn get_service_selector(service: &Service) -> String {
    service.metadata.labels
        .get(NAME_LABEL)
        .cloned()
        .unwrap_or(String::new())
}

#[allow(dead_code)]
fn get_pod_by_label(pods: &[Pod], label: &str) -> Option<Pod> {
    pods
        .iter()
        .find(|p| p.metadata.labels
            .get(NAME_LABEL)
            .unwrap_or(&String::new()) == label)
        .cloned()
}

#[allow(dead_code)]
fn is_pod_available(pod: Option<Pod>) -> bool {
    pod.is_some_and(|p| p.status.phase == "Running")
}
