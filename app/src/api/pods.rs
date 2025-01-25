use leptos::prelude::ServerFnError;
use leptos::server;

use crate::domain::pod::*;

#[server(GetPods, "/api/pods")]
pub async fn get_pods() -> Result<Vec<Pod>, ServerFnError> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let token = crate::api::utils::get_api_token();

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!("https://{server_host}:6443/api/v1/pods"))
        .bearer_auth(token)
        .send()
        .await?;

    response.error_for_status_ref()?;
    Ok(parse_response(&response.text().await?).unwrap_or_default())
}

#[allow(dead_code)]
fn parse_response(response: &str) -> Result<Vec<Pod>, Box<dyn std::error::Error>> {
    let pods = serde_json::from_str::<PodsResponse>(response)?
        .items
        .iter()
        .filter(|pod| pod.metadata.namespace != "kube-system")
        .cloned()
        .collect::<Vec<Pod>>();
    Ok(pods)
}
