use domain::cluster::event::Event;
use domain::shared::response::Response;
use leptos::prelude::ServerFnError;
use leptos::server;

use crate::resource as resource_api;

#[server(GetEvents, "/api/events")]
pub async fn get_events(namespace_name: Option<String>) -> Result<Vec<Event>, ServerFnError> {
    let response = resource_api::get("Event".to_string(), namespace_name, None).await?;
    Ok(serde_json::from_str::<Response<Event>>(&response)?.items)
}
