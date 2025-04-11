use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Secret {
    #[serde(default)]
    pub data: HashMap<String, String>,
    pub metadata: Metadata,
    pub r#type: String,
}
