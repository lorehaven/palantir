//! `api::config`'s env parsing/defaults, run as an integration test since
//! `api`'s crate root forbids unsafe code and `std::env::set_var` needs it.

use api::config;

// Each test claims its own env var (`PALANTIR_`-prefixed, per
// `ConfigLoader`'s lookup order) and asserts default-then-override within a
// single test function, so parallel test threads never observe another
// test's in-flight set/unset of the same var.

#[test]
fn server_host_falls_back_then_reads_the_prefixed_env_var() {
    assert_eq!(config::server_host(), "localhost");
    unsafe {
        std::env::set_var("PALANTIR_SERVER_HOST", "k8s.example.internal");
    }
    assert_eq!(config::server_host(), "k8s.example.internal");
    unsafe {
        std::env::remove_var("PALANTIR_SERVER_HOST");
    }
}

#[test]
fn kubernetes_token_path_falls_back_then_reads_the_prefixed_env_var() {
    assert_eq!(
        config::kubernetes_token_path(),
        "/var/run/secrets/kubernetes.io/serviceaccount/token"
    );
    unsafe {
        std::env::set_var("PALANTIR_KUBERNETES_TOKEN_PATH", "/tmp/token");
    }
    assert_eq!(config::kubernetes_token_path(), "/tmp/token");
    unsafe {
        std::env::remove_var("PALANTIR_KUBERNETES_TOKEN_PATH");
    }
}

#[test]
fn additional_services_json_falls_back_then_reads_the_prefixed_env_var() {
    assert_eq!(config::additional_services_json(), "[]");
    unsafe {
        std::env::set_var("PALANTIR_ADDITIONAL_SERVICES", "[{\"name\":\"x\"}]");
    }
    assert_eq!(config::additional_services_json(), "[{\"name\":\"x\"}]");
    unsafe {
        std::env::remove_var("PALANTIR_ADDITIONAL_SERVICES");
    }
}
