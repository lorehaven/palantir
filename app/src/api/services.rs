use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::pods::*;
use crate::domain::pod::Pod;
use crate::domain::service::*;

const NAME_LABEL: &str = "app.kubernetes.io/name";

#[server(GetServices, "/api/services")]
pub async fn get_services() -> Result<Vec<ServiceEntry>, ServerFnError> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let token = crate::api::utils::get_api_token();

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!("https://{server_host}:6443/api/v1/services"))
        .bearer_auth(token)
        .send()
        .await?;
    response.error_for_status_ref()?;
    Ok(parse_response(&response.text().await?, &get_pods().await?).unwrap_or_default())
}

#[allow(dead_code)]
fn parse_response(response: &str, pods: &[Pod]) -> Result<Vec<ServiceEntry>, Box<dyn std::error::Error>> {
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
            s.spec.ports.clone().into_iter().map(move |p| ServiceEntry {
                name: format_service_name(&p.name),
                url: format!("http://{server_host}:{}", p.port.unwrap()),
                url_display: format!("{server_dns_name}:{}", p.port.unwrap()),
                available: is_pod_available(get_pod_by_label(pods, &selector)),
            })
        })
        .collect::<Vec<ServiceEntry>>();

    let pihole_pod = get_pod_by_label(pods, "pihole");
    let wireguard_pod = get_pod_by_label(pods, "wireguard");
    services.extend([
        ServiceEntry {
            name: format_service_name("PiHole Web UI"),
            url: format!("http://{server_host}:32000/admin"),
            url_display: format!("{server_dns_name}:32000/admin"),
            available: is_pod_available(pihole_pod),
        },
        ServiceEntry {
            name: format_service_name("Wireguard Web UI"),
            url: format!("http://{server_host}:51821"),
            url_display: format!("{server_dns_name}:51821"),
            available: is_pod_available(wireguard_pod),
        },
        ServiceEntry {
            name: format_service_name("Cockpit Web UI"),
            url: format!("http://{server_host}:9090"),
            url_display: format!("{server_dns_name}:9090"),
            available: true,
        }
    ]);

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
    service.clone().metadata.labels
        .get(NAME_LABEL)
        .cloned()
        .unwrap_or("".to_string())
        .clone()
}

#[allow(dead_code)]
fn get_pod_by_label(pods: &[Pod], label: &str) -> Option<Pod> {
    pods
        .iter()
        .find(|p| p.metadata.labels
            .get("app.kubernetes.io/name")
            .unwrap_or(&String::new()) == label)
        .cloned()
}

#[allow(dead_code)]
fn is_pod_available(pod: Option<Pod>) -> bool {
    pod.map(|p| p.status.phase == "Running").unwrap_or(false)
}
