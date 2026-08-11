//! Login and logout belong to gatehouse; palantir only hands the browser
//! over and accepts it back. Mirrors every other forge actix service's
//! `routers/ui/pages/auth.rs` (e.g. `sage-service`) - there's no local login
//! form here on purpose.
//!
//! Registered with full literal paths rather than nested in a
//! `web::scope("/ui")`, deliberately: the protected Leptos pages also live
//! under `/ui/*`, and a sibling scope claiming that same prefix is exactly
//! the actix routing collision `auth_gate`'s doc comment describes - so
//! these are flat services, same as the Leptos routes are.

use actix_web::{get, web, HttpRequest, Responder};
use quench_auth::actix::domain::sso_client::SsoConfig;
use quench_auth::actix::routers::ui::pages::auth::{
    auth_callback, login_delegation, logout_delegation,
};

#[get("/ui/login")]
pub async fn login(req: HttpRequest, sso: web::Data<SsoConfig>) -> impl Responder {
    login_delegation(&req, &sso)
}

#[get("/ui/auth/callback")]
pub async fn callback(req: HttpRequest, sso: web::Data<SsoConfig>) -> impl Responder {
    auth_callback(&req, &sso).await
}

#[get("/ui/logout")]
pub async fn logout(req: HttpRequest) -> impl Responder {
    logout_delegation(&req)
}
