use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::api::utils::kube_api_request;
#[allow(unused_imports)]
use crate::domain::event::{Event, EventsResponse};

#[server(GetEvents, "/api/events")]
pub async fn get_events() -> Result<Vec<Event>, ServerFnError> {
    let response = kube_api_request("events".to_string()).await?;
    Ok(serde_json::from_str::<EventsResponse>(&response)?.items)
}

#[server(GetEventsByNamespaceName, "/api/events/by-namespace/:namespace_name")]
pub async fn get_events_by_namespace_name(namespace_name: String) -> Result<Vec<Event>, ServerFnError> {
    let response = kube_api_request("events".to_string()).await?;
    let pods = serde_json::from_str::<EventsResponse>(&response)?.items
        .into_iter()
        .filter(|e| e.metadata.namespace == namespace_name)
        .collect();
    Ok(pods)
}
