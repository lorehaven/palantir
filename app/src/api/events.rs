use leptos::prelude::ServerFnError;
use leptos::server;

use crate::domain::event::{Event, EventsResponse};

#[server(GetEvents, "/api/events")]
pub async fn get_events() -> Result<Vec<Event>, ServerFnError> {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let token = crate::api::utils::get_api_token();

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(format!("https://{server_host}:6443/api/v1/events"))
        .bearer_auth(token)
        .send()
        .await?;

    response.error_for_status_ref()?;
    Ok(parse_response(&response.text().await?).unwrap_or_default())
}

#[allow(dead_code)]
fn parse_response(response: &str) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let nodes = serde_json::from_str::<EventsResponse>(response)?.items;
    Ok(nodes)
}
