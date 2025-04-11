use serde::{Deserialize, Serialize};

use crate::shared::metadata::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Ingress {
    pub metadata: Metadata,
    pub spec: IngressSpec,
    pub status: IngressStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IngressSpec {
    #[serde(default, rename = "ingressClassName")]
    pub ingress_class_name: String,
    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Rule {
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub http: Http,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Http {
    #[serde(default)]
    pub paths: Vec<Path>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Path {
    #[serde(default)]
    pub backend: Backend,
    #[serde(default)]
    pub path: String,
    #[serde(default, rename = "pathType")]
    pub path_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Backend {
    #[serde(default)]
    pub service: Service,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Service {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub port: Port,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Port {
    #[serde(default)]
    pub number: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IngressStatus {
    #[serde(default, rename = "loadBalancer")]
    pub load_balancer: LoadBalancer,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoadBalancer {
    #[serde(default)]
    pub ingress: Option<Vec<IngressLoadBalancer>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IngressLoadBalancer {
    #[serde(default)]
    pub ip: String,
}
