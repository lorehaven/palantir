//! Per-function write-permission checks for `#[server]` functions.
//!
//! Every route Leptos auto-registers (`leptos_routes`'s own scan of
//! `server_fn::actix::server_fn_paths()`) shares one scope, so there's no
//! per-route place to hang a `RequireWrite`-style middleware the way a
//! handwritten actix route could - see that middleware's own doc comment,
//! which recommends exactly this: guard the individual route with
//! [`quench_auth::prelude::Claims::can`] instead.
#![cfg(not(target_arch = "wasm32"))]

use actix_web::web::Data;
use actix_web::{HttpMessage, HttpRequest};
use leptos::prelude::ServerFnError;
use quench_auth::prelude::{Claims, JwtConfig};

/// Confirms the caller holds this service's generic `"write"` action.
///
/// Reads the `Claims` the `Auth` middleware put in the request's
/// extensions - the same rule `RequireWrite` enforces for handwritten
/// routes. Returns `Ok(None)` rather than erroring when auth is turned off
/// (`SERVICE_AUTH_ENABLED=false`), matching `Auth`/`RequireWrite`'s own dev
/// bypass: nothing to check, so nothing is refused.
///
/// # Errors
///
/// Errors if the caller isn't authenticated at all, or is authenticated but
/// lacks the `"write"` action on this service.
pub async fn require_write() -> Result<Option<Claims>, ServerFnError> {
    let config = leptos_actix::extract::<Data<JwtConfig>>().await?;
    if !config.auth_enabled {
        return Ok(None);
    }

    let req = leptos_actix::extract::<HttpRequest>().await?;
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| ServerFnError::new("not authenticated"))?;

    if claims.can(&config.service_name, "write") {
        Ok(Some(claims))
    } else {
        Err(ServerFnError::new(
            "this action needs write permission on palantir",
        ))
    }
}
