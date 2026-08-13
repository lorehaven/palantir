use actix_web::{web, HttpRequest, HttpResponse};
use quench_auth::prelude::JwtConfig;
use quench_cache::CacheStore;

use super::{delete, html, NameFilter};

pub async fn namespaces(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    query: web::Query<NameFilter>,
) -> HttpResponse {
    html(app::pages::cluster::namespaces::render(&cache, req.path(), &query.name).await)
}

pub async fn namespaces_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<NameFilter>,
) -> HttpResponse {
    html(
        app::pages::cluster::namespaces::fragment(&cache, &query.name)
            .await
            .render(),
    )
}

pub async fn namespace(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(app::pages::cluster::namespace::render(&cache, req.path(), &name).await)
}

pub async fn namespace_delete(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    config: web::Data<JwtConfig>,
    name: web::Path<String>,
) -> HttpResponse {
    delete(
        &req,
        &cache,
        &config,
        "Namespace",
        None,
        &name,
        &format!("{}/cluster/namespaces", app::base_path::ui_base()),
    )
    .await
}

pub async fn namespace_info_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::cluster::namespace::namespace_info::fragment(&cache, &name)
            .await
            .render(),
    )
}

pub async fn namespace_pods_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::cluster::namespace::namespace_pods::fragment(&cache, &name)
            .await
            .render(),
    )
}

pub async fn namespace_events_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::cluster::namespace::namespace_events::fragment(&cache, &name)
            .await
            .render(),
    )
}

pub async fn nodes(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    query: web::Query<NameFilter>,
) -> HttpResponse {
    html(app::pages::cluster::nodes::render(&cache, req.path(), &query.name).await)
}

pub async fn nodes_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<NameFilter>,
) -> HttpResponse {
    html(
        app::pages::cluster::nodes::fragment(&cache, &query.name)
            .await
            .render(),
    )
}

pub async fn node(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(app::pages::cluster::node::render(&cache, req.path(), &name).await)
}

pub async fn node_info_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::cluster::node::node_info::fragment(&cache, &name)
            .await
            .render(),
    )
}

pub async fn node_conditions_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::cluster::node::node_conditions::fragment(&cache, &name)
            .await
            .render(),
    )
}

pub async fn node_pods_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::cluster::node::node_pods::fragment(&cache, &name)
            .await
            .render(),
    )
}
