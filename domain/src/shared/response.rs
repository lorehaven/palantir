use serde::{Deserialize, Serialize};

use crate::shared::metadata::ResponseMetadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response<T> {
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub metadata: ResponseMetadata,
    pub items: Vec<T>,
}
