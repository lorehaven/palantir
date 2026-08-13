//! Per-call write-permission checks for actions that mutate cluster state
//! (delete/scale/apply).
//!
//! Palantir's own `auth_gate` middleware (`server/src/auth_gate.rs`) already
//! puts the caller's `Claims` into the request extensions for every
//! protected route; handlers pull them out and pass them here alongside the
//! `JwtConfig` they already hold, rather than this module reaching back into
//! the request itself.

use quench_auth::prelude::{Claims, JwtConfig};

/// Confirms the caller holds this service's generic `"write"` action.
///
/// Returns `Ok(None)` rather than erroring when auth is turned off
/// (`SERVICE_AUTH_ENABLED=false`), matching the `Auth` middleware's own dev
/// bypass: nothing to check, so nothing is refused.
///
/// # Errors
///
/// Errors if the caller isn't authenticated at all, or is authenticated but
/// lacks the `"write"` action on this service.
pub fn require_write(
    config: &JwtConfig,
    claims: Option<&Claims>,
) -> anyhow::Result<Option<Claims>> {
    if !config.auth_enabled {
        return Ok(None);
    }

    let claims = claims.ok_or_else(|| anyhow::anyhow!("not authenticated"))?;

    if claims.can(&config.service_name, "write") {
        Ok(Some(claims.clone()))
    } else {
        Err(anyhow::anyhow!(
            "this action needs write permission on palantir"
        ))
    }
}
