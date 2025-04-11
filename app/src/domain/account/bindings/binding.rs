use serde::{Deserialize, Serialize};

use crate::domain::account::bindings::{BaseRoleBinding, RoleBindingModel};
use crate::domain::shared::metadata::Metadata;
use crate::pages::utils::shared::time::time_until_now;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RoleBinding {
    pub metadata: Metadata,
    #[serde(default, rename = "roleRef")]
    pub role_ref: RoleRef,
    #[serde(default)]
    pub subjects: Vec<Subject>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RoleRef {
    #[serde(rename = "apiGroup")]
    pub api_group: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Subject {
    #[serde(default, rename = "apiGroup")]
    pub api_group: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
}

impl BaseRoleBinding for RoleBinding {
    fn get_name(&self) -> String {
        self.metadata.name.clone()
    }

    fn to_model(&self) -> RoleBindingModel {
        let namespace =
            if self.metadata.namespace.is_empty() { "All Namespaces".to_string() }
            else { self.metadata.namespace.to_string() };
        RoleBindingModel {
            r#type: "RoleBinding".to_string(),
            name: self.metadata.name.clone(),
            namespace,
            age: time_until_now(&self.clone().metadata.creation_timestamp.unwrap_or_default()),
        }
    }
}
