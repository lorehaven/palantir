use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::domain::account::roles::{BaseRole, RoleModel};
use crate::domain::shared::metadata::{Metadata, ResponseMetadata};
use crate::pages::utils::shared::time::time_until_now;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClusterRolesResponse {
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub metadata: ResponseMetadata,
    pub items: Vec<ClusterRole>,
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
    pub cluster_role_selectors: Vec<ClusterRoleSelector>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClusterRoleSelector {
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

impl BaseRole for ClusterRole {
    fn get_name(&self) -> String {
        self.metadata.name.clone()
    }

    fn to_model(&self) -> RoleModel {
        let namespace =
            if self.metadata.namespace.is_empty() { "All Namespaces".to_string() }
            else { self.metadata.namespace.to_string() };
        RoleModel {
            r#type: "ClusterRole".to_string(),
            name: self.metadata.name.clone(),
            namespace,
            age: time_until_now(&self.clone().metadata.creation_timestamp.unwrap_or_default()),
        }
    }
}
