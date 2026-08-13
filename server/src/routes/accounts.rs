use actix_web::{web, HttpRequest, HttpResponse};
use quench_auth::prelude::JwtConfig;
use quench_cache::CacheStore;
use serde::Deserialize;

use super::{delete, html};

#[derive(Deserialize)]
pub struct Filter {
    #[serde(default = "all_namespaces")]
    namespace: String,
    #[serde(default)]
    name: String,
}

fn all_namespaces() -> String {
    "All Namespaces".to_string()
}

pub async fn serviceaccounts(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    query: web::Query<Filter>,
) -> HttpResponse {
    html(app::pages::accounts::render(&cache, req.path(), &query.namespace, &query.name).await)
}

pub async fn serviceaccounts_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<Filter>,
) -> HttpResponse {
    html(
        app::pages::accounts::fragment(&cache, &query.namespace, &query.name)
            .await
            .render(),
    )
}

pub async fn serviceaccount(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::accounts::serviceaccount::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn serviceaccount_delete(
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
        "ServiceAccount",
        Some(&namespace),
        &name,
        &format!("{}/accounts", app::base_path::ui_base()),
    )
    .await
}

pub async fn serviceaccount_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::accounts::serviceaccount::fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn secrets(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    query: web::Query<Filter>,
) -> HttpResponse {
    html(
        app::pages::accounts::secrets::render(&cache, req.path(), &query.namespace, &query.name)
            .await,
    )
}

pub async fn secrets_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<Filter>,
) -> HttpResponse {
    html(
        app::pages::accounts::secrets::fragment(&cache, &query.namespace, &query.name)
            .await
            .render(),
    )
}

pub async fn secret(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::accounts::secret::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn secret_delete(
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
        "Secret",
        Some(&namespace),
        &name,
        &format!("{}/accounts/secrets", app::base_path::ui_base()),
    )
    .await
}

pub async fn secret_info_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::accounts::secret::info_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn secret_data_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::accounts::secret::data_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn roles(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    query: web::Query<super::NameFilter>,
) -> HttpResponse {
    html(app::pages::accounts::roles::render(&cache, req.path(), &query_name(&query)).await)
}

pub async fn roles_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<super::NameFilter>,
) -> HttpResponse {
    html(
        app::pages::accounts::roles::fragment(&cache, &query_name(&query))
            .await
            .render(),
    )
}

fn query_name(query: &super::NameFilter) -> String {
    query.name.clone()
}

pub async fn role(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::accounts::role::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn role_delete(
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
        "Role",
        Some(&namespace),
        &name,
        &format!("{}/accounts/roles", app::base_path::ui_base()),
    )
    .await
}

pub async fn role_info_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::accounts::role::info_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn role_rules_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::accounts::role::rules_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn clusterrole(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(app::pages::accounts::clusterrole::render(&cache, req.path(), &name).await)
}

pub async fn clusterrole_delete(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    config: web::Data<JwtConfig>,
    name: web::Path<String>,
) -> HttpResponse {
    delete(
        &req,
        &cache,
        &config,
        "ClusterRole",
        None,
        &name,
        &format!("{}/accounts/roles", app::base_path::ui_base()),
    )
    .await
}

pub async fn clusterrole_info_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::accounts::clusterrole::info_fragment(&cache, &name)
            .await
            .render(),
    )
}

pub async fn clusterrole_rules_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::accounts::clusterrole::rules_fragment(&cache, &name)
            .await
            .render(),
    )
}

pub async fn bindings(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    query: web::Query<super::NameFilter>,
) -> HttpResponse {
    html(app::pages::accounts::bindings::render(&cache, req.path(), &query_name(&query)).await)
}

pub async fn bindings_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<super::NameFilter>,
) -> HttpResponse {
    html(
        app::pages::accounts::bindings::fragment(&cache, &query_name(&query))
            .await
            .render(),
    )
}

pub async fn binding(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::accounts::binding::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn binding_delete(
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
        "RoleBinding",
        Some(&namespace),
        &name,
        &format!("{}/accounts/bindings", app::base_path::ui_base()),
    )
    .await
}

pub async fn binding_info_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::accounts::binding::info_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn binding_subjects_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::accounts::binding::subjects_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn clusterbinding(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(app::pages::accounts::clusterbinding::render(&cache, req.path(), &name).await)
}

pub async fn clusterbinding_delete(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    config: web::Data<JwtConfig>,
    name: web::Path<String>,
) -> HttpResponse {
    delete(
        &req,
        &cache,
        &config,
        "ClusterRoleBinding",
        None,
        &name,
        &format!("{}/accounts/bindings", app::base_path::ui_base()),
    )
    .await
}

pub async fn clusterbinding_info_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::accounts::clusterbinding::info_fragment(&cache, &name)
            .await
            .render(),
    )
}

pub async fn clusterbinding_subjects_fragment(
    cache: web::Data<CacheStore>,
    name: web::Path<String>,
) -> HttpResponse {
    html(
        app::pages::accounts::clusterbinding::subjects_fragment(&cache, &name)
            .await
            .render(),
    )
}
