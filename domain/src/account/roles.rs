use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::shared::metadata::Metadata;
use crate::utils::time::time_until_now;

pub trait BaseRole: std::fmt::Debug + Sync + Send {
    fn get_name(&self) -> String;
    fn to_model(&self) -> RoleModel;
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RoleModel {
    pub r#type: String,
    pub name: String,
    pub namespace: String,
    pub age: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Role {
    pub metadata: Metadata,
    pub rules: Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClusterRole {
    #[serde(default, rename = "aggregationRule")]
    pub aggregation_rule: AggregationRule,
    pub metadata: Metadata,
    pub rules: Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AggregationRule {
    #[serde(default, rename = "clusterRoleSelectors")]
    pub cluster_role_selectors: Vec<Selector>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Selector {
    #[serde(default, rename = "matchLabels")]
    pub match_labels: HashMap<String, String>,
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

macro_rules! impl_base_role {
    ($type:ty, $kind:expr) => {
        impl BaseRole for $type {
            fn get_name(&self) -> String {
                self.metadata.name.clone()
            }

            fn to_model(&self) -> RoleModel {
                let namespace = if self.metadata.namespace.is_empty() {
                    "All Namespaces".to_string()
                } else {
                    self.metadata.namespace.clone()
                };

                RoleModel {
                    r#type: $kind.to_string(),
                    name: self.metadata.name.clone(),
                    namespace,
                    age: time_until_now(&self.metadata.creation_timestamp.clone().unwrap_or_default()),
                }
            }
        }
    };
}

impl_base_role!(Role, "Role");
impl_base_role!(ClusterRole, "ClusterRole");
