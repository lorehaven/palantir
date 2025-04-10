use std::fmt::Debug;
use serde::{Deserialize, Serialize};

pub mod clusterrole;
pub mod role;

pub trait BaseRole: Debug + Sync + Send {
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
