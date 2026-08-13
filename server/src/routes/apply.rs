use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use api::utils::ApiMode;
use quench_auth::prelude::{Claims, JwtConfig};
use quench_cache::CacheStore;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApplyForm {
    yaml: String,
}

pub async fn create(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    config: web::Data<JwtConfig>,
    form: web::Form<ApplyForm>,
) -> HttpResponse {
    apply(&req, &cache, &config, &form.yaml, ApiMode::Post).await
}

pub async fn update(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    config: web::Data<JwtConfig>,
    form: web::Form<ApplyForm>,
) -> HttpResponse {
    apply(&req, &cache, &config, &form.yaml, ApiMode::Put).await
}

async fn apply(
    req: &HttpRequest,
    cache: &CacheStore,
    config: &JwtConfig,
    yaml: &str,
    mode: ApiMode,
) -> HttpResponse {
    let claims = req.extensions().get::<Claims>().cloned();

    let json = match yaml_to_json(yaml) {
        Ok(json) => json,
        Err(err) => {
            tracing::warn!("failed to parse applied yaml: {err}");
            return HttpResponse::BadRequest().finish();
        }
    };

    match api::apply::apply(cache, config, claims.as_ref(), json, mode).await {
        // Not a redirect: the dialog can be reopened from any page, so
        // there's no single "back to" URL to send - `HX-Refresh` just
        // reloads whatever page the request came from.
        Ok(_) => HttpResponse::Ok()
            .insert_header(("HX-Refresh", "true"))
            .finish(),
        Err(err) => {
            tracing::warn!("failed to apply resource: {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

fn yaml_to_json(yaml: &str) -> anyhow::Result<String> {
    let value = serde_yaml::from_str::<serde_yaml::Value>(yaml)?;
    Ok(serde_json::to_string(&value)?)
}
