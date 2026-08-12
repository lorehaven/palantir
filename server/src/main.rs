use std::sync::Arc;

use actix_files::{Files, NamedFile};
use actix_web::dev::HttpServiceFactory;
use actix_web::middleware::from_fn;
use actix_web::web;
use app::*;
use leptos::prelude::*;
use leptos_actix::LeptosRoutes;
use quench_auth::actix::domain::session::SessionDb;
use quench_auth::actix::domain::sso_client::SsoConfig;
use quench_auth::prelude::JwtConfig;
use quench_cache::CacheStore;

mod auth_gate;
mod auth_routes;
mod strip_base_path;
pub mod ws;

/// Everything the app needs registered as `app_data`, gathered once at boot.
/// `SessionDb` backs `auth_gate`'s session-liveness check; `CacheStore`
/// backs `/ws/exec`'s ticket mint/redeem (see `api::ws_ticket`).
#[derive(Clone)]
struct AuthState {
    jwt_config: JwtConfig,
    sso_config: SsoConfig,
    session_db: Arc<SessionDb>,
    cache_store: CacheStore,
}

/// Mounted at the true root by `quench_starter::actix::serve()`, alongside
/// its own UI/health redirects. Palantir has nothing that needs to live
/// outside `BASE_PATH`, so this is intentionally empty - it exists only to
/// satisfy `serve()`'s `root_module` parameter (same pattern as gatehouse's
/// own `root_scope()`).
fn root_scope() -> impl HttpServiceFactory {
    web::scope("")
}

/// Everything palantir actually serves, mounted under `BASE_PATH` by
/// `serve()`. `leptos_actix::LeptosRoutes` isn't implemented for `Scope`
/// directly, only for `App`/`&mut ServiceConfig` - `Scope::configure()` is
/// the bridge (confirmed by a throwaway compile spike before this was
/// written).
///
/// One flat scope, wrapped in `auth_gate` rather than split into a public
/// and an `Auth`-wrapped scope: login/callback/logout live under `/ui`
/// alongside every protected page, and actix's scope router commits to
/// whichever sibling scope's prefix matches first, never trying the other -
/// see `auth_gate`'s own doc comment for the confirmed failure mode.
fn app_scope(leptos_options: LeptosOptions, auth: AuthState) -> impl HttpServiceFactory {
    let site_root = leptos_options.site_root.to_string();
    // `app::base_path::during_route_enumeration` makes `<Router>`'s base
    // report a bare `/ui` for just this call, not the full `BASE_PATH`-
    // prefixed one real requests need - `quench_starter::actix::serve()`
    // already nests this whole scope under `BASE_PATH`, so the route
    // strings registered here have to stay relative to it. See
    // `app::base_path`'s own doc comment for the full reasoning.
    let routes = app::base_path::during_route_enumeration(|| {
        leptos_actix::generate_route_list(web_app::WebApp)
    });
    let shell_options = leptos_options.clone();

    web::scope("")
        .app_data(web::Data::new(leptos_options))
        .app_data(web::Data::new(auth.jwt_config))
        .app_data(web::Data::new(auth.sso_config))
        .app_data(web::Data::new(auth.session_db))
        .app_data(web::Data::new(auth.cache_store))
        // Registered in reverse-execution order: actix runs the *last*
        // `.wrap()` first, so `auth_gate` (which needs the full,
        // BASE_PATH-prefixed path for its own redirect-back URL) has to
        // stay outermost - added last - with the server-fn path-stripping
        // fix running only after it, right before the actual handler.
        .wrap(from_fn(strip_base_path::strip_base_path_for_server_fns))
        .wrap(from_fn(auth_gate::auth_gate))
        .service(Files::new("/pkg", format!("{site_root}/pkg")))
        .service(favicon)
        .service(auth_routes::login)
        .service(auth_routes::callback)
        .service(auth_routes::logout)
        .route("/ws/exec", web::get().to(ws::exec_ws_handler))
        .configure(move |cfg| {
            cfg.leptos_routes(routes.clone(), {
                let shell_options = shell_options.clone();
                move || shell::shell(shell_options.clone())
            });
        })
}

#[actix_web::get("favicon.ico")]
async fn favicon(leptos_options: web::Data<LeptosOptions>) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(format!(
        "{}/favicon.ico",
        leptos_options.site_root
    ))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

    let auth = AuthState {
        jwt_config: JwtConfig::init().await,
        sso_config: SsoConfig::init(),
        session_db: SessionDb::from_env()
            .await
            .expect("session store unavailable (is REDIS_URL reachable?)"),
        cache_store: CacheStore::from_env("palantir")
            .await
            .expect("cache store unavailable (is REDIS_URL reachable?)"),
    };

    quench_starter::actix::serve(
        root_scope,
        move || app_scope(leptos_options.clone(), auth.clone()),
        None,
        async {},
    )
    .await
}
