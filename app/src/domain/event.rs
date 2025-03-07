use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventsResponse {
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub metadata: ResponseMetadata,
    pub items: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResponseMetadata {
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub metadata: Metadata,
    #[serde(rename = "involvedObject")]
    pub involved_object: InvolvedObject,
    pub reason: String,
    pub message: String,
    pub source: Source,
    #[serde(rename = "firstTimestamp")]
    pub first_timestamp: Option<String>,
    #[serde(rename = "lastTimestamp")]
    pub last_timestamp: Option<String>,
    #[serde(default)]
    pub count: usize,
    pub r#type: String,
    #[serde(default)]
    #[serde(rename = "eventTime")]
    pub event_time: Option<String>,
    #[serde(rename = "reportingComponent")]
    pub reporting_component: String,
    #[serde(rename = "reportingInstance")]
    pub reporting_instance: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
    pub uid: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvolvedObject {
    pub kind: String,
    pub namespace: String,
    pub name: String,
    #[serde(default)]
    pub uid: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(default)]
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    #[serde(default)]
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub component: String,
    #[serde(default)]
    pub host: String,
}
