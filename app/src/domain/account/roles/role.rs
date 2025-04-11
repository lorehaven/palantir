use serde::{Deserialize, Serialize};

use crate::domain::account::roles::{BaseRole, RoleModel};
use crate::domain::shared::metadata::Metadata;
use crate::pages::utils::shared::time::time_until_now;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Role {
    pub metadata: Metadata,
    pub rules: Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Rule {
    #[serde(default, rename = "apiGroups")]
    pub api_groups: Vec<String>,
    #[serde(default, rename = "resourceNames")]
    pub resource_names: Vec<String>,
    #[serde(default)]
    pub resources: Vec<String>,
    #[serde(default)]
    pub verbs: Vec<String>,
}

impl BaseRole for Role {
    fn get_name(&self) -> String {
        self.metadata.name.clone()
    }

    fn to_model(&self) -> RoleModel {
        let namespace =
            if self.metadata.namespace.is_empty() { "All Namespaces".to_string() }
            else { self.metadata.namespace.to_string() };
        RoleModel {
            r#type: "Role".to_string(),
            name: self.metadata.name.clone(),
            namespace,
            age: time_until_now(&self.clone().metadata.creation_timestamp.unwrap_or_default()),
        }
    }
}
