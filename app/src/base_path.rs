//! `BASE_PATH` is a per-deployment runtime env var (e.g. a reverse proxy
//! mounting palantir under `/palantir` rather than at the root).
//!
//! Every server-rendered link/asset URL this crate builds needs it as a
//! prefix - `quench_starter::actix::serve()` already nests the whole app
//! under it on the routing side, but that doesn't rewrite the HTML this
//! crate emits, so it has to be threaded through explicitly here.

pub fn base_path() -> String {
    normalize(&std::env::var("BASE_PATH").unwrap_or_default())
}

fn normalize(raw: &str) -> String {
    let trimmed = raw.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        String::new()
    } else if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

/// Prefix for every palantir UI link: `BASE_PATH` (if any) plus the app's
/// own `/ui` mount point.
pub fn ui_base() -> String {
    format!("{}/ui", base_path())
}
