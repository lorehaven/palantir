use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use quench_auth::prelude::{Claims, JwtConfig};
use quench_cache::CacheStore;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScaleForm {
    resource_type: String,
    #[serde(default)]
    namespace: Option<String>,
    name: String,
    replicas: i64,
}

pub async fn update(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    config: web::Data<JwtConfig>,
    form: web::Form<ScaleForm>,
) -> HttpResponse {
    let claims = req.extensions().get::<Claims>().cloned();
    let result = api::resource::scale(
        &cache,
        &config,
        claims.as_ref(),
        &form.resource_type,
        form.namespace.clone(),
        Some(form.name.clone()),
        form.replicas,
    )
    .await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .insert_header(("HX-Refresh", "true"))
            .finish(),
        Err(err) => {
            tracing::warn!(
                "failed to scale {} {}: {err}",
                form.resource_type,
                form.name
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
