use actix_web::{web, HttpRequest, HttpResponse};
use quench_auth::prelude::JwtConfig;
use quench_cache::CacheStore;
use serde::Deserialize;

use super::{delete, html, NameFilter};

#[derive(Deserialize)]
pub struct ClaimsFilter {
    #[serde(default = "all_namespaces")]
    namespace: String,
    #[serde(default)]
    name: String,
}

fn all_namespaces() -> String {
    "All Namespaces".to_string()
}

pub async fn storageclasses(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    query: web::Query<NameFilter>,
) -> HttpResponse {
    html(app::pages::storage::render(&cache, req.path(), &query.name).await)
}

pub async fn storageclasses_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<NameFilter>,
) -> HttpResponse {
    html(
        app::pages::storage::fragment(&cache, &query.name)
            .await
            .render(),
    )
}

pub async fn storageclass(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(app::pages::storage::storageclass::render(&cache, req.path(), &name).await)
}

pub async fn storageclass_delete(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    config: web::Data<JwtConfig>,
    name: web::Path<String>,
) -> HttpResponse {
    delete(
        &req,
        &cache,
        &config,
        "StorageClass",
        None,
        &name,
        &format!("{}/storage", app::base_path::ui_base()),
    )
    .await
}

pub async fn storageclass_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::storage::storageclass::fragment(&cache, &name)
            .await
            .render(),
    )
}

pub async fn volumes(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    query: web::Query<NameFilter>,
) -> HttpResponse {
    html(app::pages::storage::volumes::render(&cache, req.path(), &query.name).await)
}

pub async fn volumes_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<NameFilter>,
) -> HttpResponse {
    html(
        app::pages::storage::volumes::fragment(&cache, &query.name)
            .await
            .render(),
    )
}

pub async fn volume(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(app::pages::storage::volume::render(&cache, req.path(), &name).await)
}

pub async fn volume_delete(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    config: web::Data<JwtConfig>,
    name: web::Path<String>,
) -> HttpResponse {
    delete(
        &req,
        &cache,
        &config,
        "PersistentVolume",
        None,
        &name,
        &format!("{}/storage/volumes", app::base_path::ui_base()),
    )
    .await
}

pub async fn volume_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::storage::volume::fragment(&cache, &name)
            .await
            .render(),
    )
}

pub async fn claims(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    query: web::Query<ClaimsFilter>,
) -> HttpResponse {
    html(
        app::pages::storage::claims::render(&cache, req.path(), &query.namespace, &query.name)
            .await,
    )
}

pub async fn claims_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<ClaimsFilter>,
) -> HttpResponse {
    html(
        app::pages::storage::claims::fragment(&cache, &query.namespace, &query.name)
            .await
            .render(),
    )
}

pub async fn claim(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::storage::claim::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn claim_delete(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    config: web::Data<JwtConfig>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    delete(
        &req,
        &cache,
        &config,
        "PersistentVolumeClaim",
        Some(&namespace),
        &name,
        &format!("{}/storage/claims", app::base_path::ui_base()),
    )
    .await
}

pub async fn claim_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::storage::claim::fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}
