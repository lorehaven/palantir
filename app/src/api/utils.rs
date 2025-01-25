#[allow(dead_code)]
pub fn get_api_token() -> String {
    std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/token")
        .expect("token file is missing.")
        .trim()
        .to_string()
}
