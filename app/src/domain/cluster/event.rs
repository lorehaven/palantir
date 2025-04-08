use serde::{Deserialize, Serialize};

use crate::domain::shared::metadata::{Metadata, ResponseMetadata};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventsResponse {
    pub kind: String,
    #[serde(default, rename = "apiVersion")]
    pub api_version: String,
    pub metadata: ResponseMetadata,
    #[serde(default)]
    pub items: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub metadata: Metadata,
    #[serde(default, rename = "involvedObject")]
    pub involved_object: InvolvedObject,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub source: Source,
    #[serde(default, rename = "firstTimestamp")]
    pub first_timestamp: Option<String>,
    #[serde(default, rename = "lastTimestamp")]
    pub last_timestamp: Option<String>,
    #[serde(default)]
    pub count: usize,
    pub r#type: String,
    #[serde(default, rename = "eventTime")]
    pub event_time: Option<String>,
    #[serde(default, rename = "reportingComponent")]
    pub reporting_component: String,
    #[serde(default, rename = "reportingInstance")]
    pub reporting_instance: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InvolvedObject {
    pub kind: String,
    pub namespace: String,
    pub name: String,
    #[serde(default)]
    pub uid: String,
    #[serde(default, rename = "apiVersion")]
    pub api_version: String,
    #[serde(default, rename = "resourceVersion")]
    pub resource_version: String,
    #[serde(default, rename = "fieldPath")]
    pub field_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Source {
    #[serde(default)]
    pub component: String,
    #[serde(default)]
    pub host: String,
}
