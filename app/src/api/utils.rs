const DEFAULT_TOKEN_PATH: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";

#[allow(dead_code)]
pub fn get_api_token() -> String {
    let token_path = std::env::var("KUBERNETES_TOKEN_PATH").unwrap_or_else(|_| DEFAULT_TOKEN_PATH.to_string());
    std::fs::read_to_string(token_path)
        .expect("token file is missing.")
        .trim()
        .to_string()
}
