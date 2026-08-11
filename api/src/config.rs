use quench_config::ConfigLoader;

/// Default in-cluster location of a pod's own K8s `ServiceAccount` token,
/// overridable for local dev (`run.sh` points it at `/tmp/token`).
const DEFAULT_TOKEN_PATH: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";

fn loader() -> ConfigLoader {
    ConfigLoader::new("PALANTIR")
}

/// The Kubernetes API server's host, reached over `https://{host}:{server_port()}`.
pub fn server_host() -> String {
    loader().env_string("SERVER_HOST", "localhost")
}

/// `SERVER_HOST`'s port.
///
/// Defaults to `6443` - a raw API server address (the typical local-dev
/// shape: minikube/kind/k3s expose it directly on that port). A K8s
/// `Service` DNS name (e.g. `kubernetes.default.svc`, what the in-cluster
/// deployment uses) is a different shape entirely: the `Service` itself
/// listens on `443` and forwards to the real API server's own port, which is
/// what `6443` actually is - so this has to be overridable per deployment
/// rather than assumed.
pub fn server_port() -> String {
    loader().env_string("SERVER_PORT", "6443")
}

/// Display-only name shown for services discovered via `SERVER_HOST`.
pub fn server_dns_name() -> String {
    loader().env_string("SERVER_DNS_NAME", "localhost")
}

pub fn kubernetes_token_path() -> String {
    loader().env_string("KUBERNETES_TOKEN_PATH", DEFAULT_TOKEN_PATH)
}

/// Raw JSON array of `{name, url, url_display, available}` entries, parsed
/// by callers into `ServiceEntry`. Kept as a string here since `ConfigLoader`
/// has no JSON-array env accessor.
pub fn additional_services_json() -> String {
    loader().env_string("ADDITIONAL_SERVICES", "[]")
}
