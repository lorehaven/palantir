use std::fmt::Debug;
use serde::{Deserialize, Serialize};

pub mod clusterbinding;
pub mod binding;

pub trait BaseRoleBinding: Debug + Sync + Send {
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
