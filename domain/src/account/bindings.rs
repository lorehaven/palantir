use serde::{Deserialize, Serialize};

use crate::shared::metadata::Metadata;
use crate::utils::time::time_until_now;

pub trait BaseRoleBinding: std::fmt::Debug + Sync + Send {
    fn get_name(&self) -> String;
    fn to_model(&self) -> RoleBindingModel;
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RoleBindingModel {
    pub r#type: String,
    pub name: String,
    pub namespace: String,
    pub age: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RoleBinding {
    pub metadata: Metadata,
    #[serde(default, rename = "roleRef")]
    pub role_ref: RoleRef,
    #[serde(default)]
    pub subjects: Vec<Subject>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClusterRoleBinding {
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

macro_rules! impl_base_binding {
    ($type:ty, $kind:expr) => {
        impl BaseRoleBinding for $type {
            fn get_name(&self) -> String {
                self.metadata.name.clone()
            }

            fn to_model(&self) -> RoleBindingModel {
                let namespace = if self.metadata.namespace.is_empty() {
                    "All Namespaces".to_string()
                } else {
                    self.metadata.namespace.clone()
                };

                RoleBindingModel {
                    r#type: $kind.to_string(),
                    name: self.metadata.name.clone(),
                    namespace,
                    age: time_until_now(
                        &self.metadata.creation_timestamp.clone().unwrap_or_default(),
                    ),
                }
            }
        }
    };
}

impl_base_binding!(RoleBinding, "RoleBinding");
impl_base_binding!(ClusterRoleBinding, "ClusterRoleBinding");
