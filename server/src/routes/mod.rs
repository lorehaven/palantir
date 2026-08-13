pub mod accounts;
pub mod apply;
pub mod cluster;
pub mod scale;
pub mod storage;
pub mod workloads;

use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use quench_auth::prelude::{Claims, JwtConfig};
use quench_cache::CacheStore;
use serde::Deserialize;

pub(crate) fn html(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

/// Shared by every plain `?name=` filter query (list pages with just a
/// search box, no namespace scoping).
#[derive(Deserialize)]
pub(crate) struct NameFilter {
    #[serde(default)]
    pub(crate) name: String,
}

/// Shared by every domain's `*_delete` handler: pulls `Claims` out of the
/// request (put there by `auth_gate` - absent when auth is disabled),
/// deletes via `api::resource::delete`, and either redirects htmx back to
/// `redirect_to` or reports the failure.
pub(crate) async fn delete(
    req: &HttpRequest,
    cache: &CacheStore,
    config: &JwtConfig,
    resource_type: &str,
    namespace: Option<&str>,
    name: &str,
    redirect_to: &str,
) -> HttpResponse {
    let claims = req.extensions().get::<Claims>().cloned();
    let result = api::resource::delete(
        cache,
        config,
        claims.as_ref(),
        resource_type,
        namespace.map(String::from),
        Some(name.to_string()),
    )
    .await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .insert_header(("HX-Redirect", redirect_to))
            .finish(),
        Err(err) => {
            tracing::warn!("failed to delete {resource_type} {name}: {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
