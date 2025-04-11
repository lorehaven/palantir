use leptos::prelude::ServerFnError;
use leptos::server;

#[allow(unused_imports)]
use crate::utils::{kube_api_request, ApiType};
use domain::cluster::event::Event;
#[allow(unused_imports)]
use domain::shared::response::Response;

#[server(GetEvents, "/api/events")]
pub async fn get_events(
    namespace_name: Option<String>,
) -> Result<Vec<Event>, ServerFnError> {
    let response = kube_api_request(ApiType::Api, "events".to_string()).await?;
    let items = serde_json::from_str::<Response<Event>>(&response)?.items
        .into_iter()
        .filter(|f| f.metadata.namespace.contains(&namespace_name.clone().unwrap_or_default()))
        .collect();
    Ok(items)
}
