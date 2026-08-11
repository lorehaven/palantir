//! A path-aware stand-in for `quench_auth::actix::middleware::auth::Auth`.
//!
//! `Auth` is a per-scope `.wrap()`, but palantir can't scope it the obvious
//! way: the public login/callback/logout routes and every protected Leptos
//! page live under the *same* `/ui` prefix, and actix's scope router commits
//! to whichever sibling scope's prefix matches first, never trying the
//! other even for a path it doesn't define -
//! <https://github.com/actix/actix-web/issues/2904> (confirmed by testing
//! here: splitting public/protected into sibling scopes silently 404'd
//! everything in the scope that lost the race, both nested and as siblings).
//!
//! This wraps the *whole* scope instead and skips enforcement by path,
//! reusing `Auth`'s own bearer/cookie/JWKS/session logic rather than
//! re-deriving it.

use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::web::Data;
use actix_web::{Error, HttpMessage, HttpResponse};
use quench_auth::actix::domain::{realm, session::SessionDb};
use quench_auth::prelude::JwtConfig;
use std::sync::Arc;

/// Static assets, the SSO round trip, and `/ws/exec` (authenticated
/// indirectly through its own single-use ticket - see `api::ws_ticket`, not
/// through this middleware).
fn is_public(path: &str) -> bool {
    path.contains("/pkg/")
        || path.ends_with("/favicon.ico")
        || path.ends_with("/ws/exec")
        || path.ends_with("/ui/login")
        || path.ends_with("/ui/auth/callback")
        || path.ends_with("/ui/logout")
}

pub async fn auth_gate<B: MessageBody + 'static>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    if is_public(req.path()) {
        return Ok(next.call(req).await?.map_into_boxed_body());
    }

    let Some(config) = req.app_data::<Data<JwtConfig>>().cloned() else {
        // No JwtConfig registered at all - nothing to enforce against.
        return Ok(next.call(req).await?.map_into_boxed_body());
    };

    if !config.auth_enabled {
        return Ok(next.call(req).await?.map_into_boxed_body());
    }

    let mut token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::to_string);

    if token.is_none() {
        if let Some(cookie) = req.cookie(&realm::session_cookie_name()) {
            token = Some(cookie.value().to_string());
        }
    }

    let Some(token) = token else {
        tracing::warn!(
            "auth_gate: no token found in header or cookie for {}",
            req.path()
        );
        return Ok({
            let res = unauthorized(&req);
            req.into_response(res)
        }
        .map_into_boxed_body());
    };

    let claims = match config.decode_claims(&token).await {
        Ok(claims) => claims,
        Err(err) => {
            tracing::warn!("auth_gate: failed to decode claims: {err:?}");
            return Ok({
                let res = unauthorized(&req);
                req.into_response(res)
            }
            .map_into_boxed_body());
        }
    };

    if !claims.allows(&config.service_name) {
        tracing::warn!(
            "auth_gate: token audience mismatch, expected {}, got {:?}",
            config.service_name,
            claims.aud
        );
        return Ok({
            let res = unauthorized(&req);
            req.into_response(res)
        }
        .map_into_boxed_body());
    }

    if let Some(session_id) = claims.sid.as_deref() {
        let active = match req.app_data::<Data<Arc<SessionDb>>>() {
            Some(session_db) => session_db
                .is_active(session_id, &claims.sub)
                .await
                .unwrap_or(false),
            None => false,
        };
        if !active {
            tracing::warn!(
                "auth_gate: session {session_id} is not active for {}",
                claims.sub
            );
            return Ok({
                let res = unauthorized(&req);
                req.into_response(res)
            }
            .map_into_boxed_body());
        }
    }

    req.extensions_mut().insert(claims);
    Ok(next.call(req).await?.map_into_boxed_body())
}

/// 401 for API callers; a redirect to gatehouse's login for browsers -
/// mirrors `Auth`'s own `unauthorized()` exactly.
fn unauthorized(req: &ServiceRequest) -> HttpResponse {
    if wants_html(req) {
        if let Some(login_url) = realm::gatehouse_login_url(Some(&req.uri().to_string())) {
            return HttpResponse::Found()
                .append_header(("Location", login_url))
                .finish();
        }
    }

    HttpResponse::Unauthorized()
        .append_header(("WWW-Authenticate", "Bearer"))
        .finish()
}

fn wants_html(req: &ServiceRequest) -> bool {
    req.headers()
        .get("Accept")
        .and_then(|accept| accept.to_str().ok())
        .is_some_and(|accept| accept.contains("text/html"))
}
