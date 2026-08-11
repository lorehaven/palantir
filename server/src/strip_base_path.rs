//! A real bug in `leptos_actix` 0.7.8, not a config gap: `handle_server_fns_with_context`
//! (its shared dispatcher for every registered server function) reads
//! `req.path()` - the *raw, full* incoming path - and looks it up directly
//! against `server_fn`'s global registry, whose keys are the *relative*
//! `#[server(..., "/api/...")]` macro paths. actix's own scope-based routing
//! already correctly dispatches the request here (it matches the individual
//! per-function route registered under the outer `BASE_PATH` scope), but
//! that dispatcher then throws that match away and redoes its own lookup
//! with the wrong (unstripped) path, which can never match once `BASE_PATH`
//! is non-root - every server function call 400s with "Could not find a
//! server function at the route ...", regardless of registration or auth
//! being correct.
//!
//! Can't patch a vendored crate, so this strips `BASE_PATH` back off `/api/*`
//! requests specifically, right before `handle_server_fns_with_context` sees
//! them - by the time any `.wrap()` middleware runs, actix's own scope
//! routing has already picked which handler to call, so rewriting the URI
//! here doesn't disturb that decision, it only changes what the handler
//! itself reads.
//!
//! Scoped to `/api/*` on purpose, not every request this scope sees: SSR
//! page rendering needs the *full* raw path (`<Router base>` matching -
//! see `app::base_path`'s own doc comment) and `auth_gate`'s redirect-back
//! URL is built from the full path too. This has to run as the innermost
//! `.wrap()` - after `auth_gate`, not before it - or it would strip the
//! path those depend on before they ever see it.

use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::Error;

pub async fn strip_base_path_for_server_fns<B: MessageBody + 'static>(
    mut req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let base_path = app::base_path::base_path();
    let api_prefix = format!("{base_path}/api");
    if !base_path.is_empty() && req.path().starts_with(&api_prefix) {
        let stripped_path = &req.path()[base_path.len()..];
        let rebuilt = match req.uri().query() {
            Some(query) => format!("{stripped_path}?{query}"),
            None => stripped_path.to_string(),
        };
        if let Ok(new_uri) = rebuilt.parse() {
            req.head_mut().uri = new_uri;
        }
    }

    Ok(next.call(req).await?.map_into_boxed_body())
}
