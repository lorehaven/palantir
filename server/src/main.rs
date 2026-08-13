use std::sync::Arc;

use actix_files::Files;
use actix_web::dev::HttpServiceFactory;
use actix_web::middleware::from_fn;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use quench_auth::actix::domain::session::SessionDb;
use quench_auth::actix::domain::sso_client::SsoConfig;
use quench_auth::prelude::JwtConfig;
use quench_cache::CacheStore;

mod auth_gate;
mod auth_routes;
mod routes;
pub mod ws;

/// Everything the app needs registered as `app_data`, gathered once at boot.
/// `SessionDb` backs `auth_gate`'s session-liveness check; `CacheStore`
/// backs `/ws/exec`'s ticket mint/redeem (see `api::ws_ticket`) and every
/// page's K8s API reads.
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
/// `serve()`.
///
/// One flat scope, wrapped in `auth_gate` rather than split into a public
/// and an `Auth`-wrapped scope: login/callback/logout live under `/ui`
/// alongside every protected page, and actix's scope router commits to
/// whichever sibling scope's prefix matches first, never trying the other -
/// see `auth_gate`'s own doc comment for the confirmed failure mode.
fn app_scope(auth: AuthState) -> impl HttpServiceFactory {
    web::scope("")
        .app_data(web::Data::new(auth.jwt_config))
        .app_data(web::Data::new(auth.sso_config))
        .app_data(web::Data::new(auth.session_db))
        .app_data(web::Data::new(auth.cache_store))
        .wrap(from_fn(auth_gate::auth_gate))
        .service(Files::new("/assets", "dist/assets"))
        .service(auth_routes::login)
        .service(auth_routes::callback)
        .service(auth_routes::logout)
        .route("/ws/exec", web::get().to(ws::exec_ws_handler))
        .route("/ui", web::get().to(dashboard))
        .route("/ui/home", web::get().to(dashboard))
        .service(
            web::resource("/ui/apply")
                .route(web::post().to(routes::apply::create))
                .route(web::put().to(routes::apply::update)),
        )
        .route("/ui/scale", web::put().to(routes::scale::update))
        .configure(cluster_routes)
        .configure(storage_routes)
        .configure(accounts_routes)
        .configure(workloads_routes)
        .route("/ui/profile", web::get().to(profile))
        .route("/ui/facade", web::get().to(facade))
        .route("/ui/facade/fragment", web::get().to(facade_fragment))
        .default_service(web::route().to(not_found))
}

fn cluster_routes(cfg: &mut web::ServiceConfig) {
    use routes::cluster as r;

    cfg.route(
        "/ui/cluster/namespaces/fragment",
        web::get().to(r::namespaces_fragment),
    )
    .route("/ui/cluster/namespaces", web::get().to(r::namespaces))
    .route(
        "/ui/cluster/namespaces/{name}/info/fragment",
        web::get().to(r::namespace_info_fragment),
    )
    .route(
        "/ui/cluster/namespaces/{name}/pods/fragment",
        web::get().to(r::namespace_pods_fragment),
    )
    .route(
        "/ui/cluster/namespaces/{name}/events/fragment",
        web::get().to(r::namespace_events_fragment),
    )
    .service(
        web::resource("/ui/cluster/namespaces/{name}")
            .route(web::get().to(r::namespace))
            .route(web::delete().to(r::namespace_delete)),
    )
    .route(
        "/ui/cluster/nodes/fragment",
        web::get().to(r::nodes_fragment),
    )
    .route("/ui/cluster/nodes", web::get().to(r::nodes))
    .route(
        "/ui/cluster/nodes/{name}/info/fragment",
        web::get().to(r::node_info_fragment),
    )
    .route(
        "/ui/cluster/nodes/{name}/conditions/fragment",
        web::get().to(r::node_conditions_fragment),
    )
    .route(
        "/ui/cluster/nodes/{name}/pods/fragment",
        web::get().to(r::node_pods_fragment),
    )
    .route("/ui/cluster/nodes/{name}", web::get().to(r::node));
}

fn storage_routes(cfg: &mut web::ServiceConfig) {
    use routes::storage as r;

    cfg.route(
        "/ui/storage/fragment",
        web::get().to(r::storageclasses_fragment),
    )
    .route("/ui/storage", web::get().to(r::storageclasses))
    .route(
        "/ui/storageclasses/{name}/fragment",
        web::get().to(r::storageclass_fragment),
    )
    .service(
        web::resource("/ui/storageclasses/{name}")
            .route(web::get().to(r::storageclass))
            .route(web::delete().to(r::storageclass_delete)),
    )
    .route(
        "/ui/storage/volumes/fragment",
        web::get().to(r::volumes_fragment),
    )
    .route("/ui/storage/volumes", web::get().to(r::volumes))
    .route(
        "/ui/storage/volumes/{name}/fragment",
        web::get().to(r::volume_fragment),
    )
    .service(
        web::resource("/ui/storage/volumes/{name}")
            .route(web::get().to(r::volume))
            .route(web::delete().to(r::volume_delete)),
    )
    .route(
        "/ui/storage/claims/fragment",
        web::get().to(r::claims_fragment),
    )
    .route("/ui/storage/claims", web::get().to(r::claims))
    .route(
        "/ui/storage/{namespace}/claims/{name}/fragment",
        web::get().to(r::claim_fragment),
    )
    .service(
        web::resource("/ui/storage/{namespace}/claims/{name}")
            .route(web::get().to(r::claim))
            .route(web::delete().to(r::claim_delete)),
    );
}

fn accounts_routes(cfg: &mut web::ServiceConfig) {
    use routes::accounts as r;

    cfg.route(
        "/ui/accounts/fragment",
        web::get().to(r::serviceaccounts_fragment),
    )
    .route("/ui/accounts", web::get().to(r::serviceaccounts))
    .route(
        "/ui/accounts/{namespace}/serviceaccounts/{name}/fragment",
        web::get().to(r::serviceaccount_fragment),
    )
    .service(
        web::resource("/ui/accounts/{namespace}/serviceaccounts/{name}")
            .route(web::get().to(r::serviceaccount))
            .route(web::delete().to(r::serviceaccount_delete)),
    )
    .route(
        "/ui/accounts/secrets/fragment",
        web::get().to(r::secrets_fragment),
    )
    .route("/ui/accounts/secrets", web::get().to(r::secrets))
    .route(
        "/ui/accounts/{namespace}/secrets/{name}/info/fragment",
        web::get().to(r::secret_info_fragment),
    )
    .route(
        "/ui/accounts/{namespace}/secrets/{name}/data/fragment",
        web::get().to(r::secret_data_fragment),
    )
    .service(
        web::resource("/ui/accounts/{namespace}/secrets/{name}")
            .route(web::get().to(r::secret))
            .route(web::delete().to(r::secret_delete)),
    )
    .route(
        "/ui/accounts/roles/fragment",
        web::get().to(r::roles_fragment),
    )
    .route("/ui/accounts/roles", web::get().to(r::roles))
    .route(
        "/ui/accounts/{namespace}/roles/{name}/info/fragment",
        web::get().to(r::role_info_fragment),
    )
    .route(
        "/ui/accounts/{namespace}/roles/{name}/rules/fragment",
        web::get().to(r::role_rules_fragment),
    )
    .service(
        web::resource("/ui/accounts/{namespace}/roles/{name}")
            .route(web::get().to(r::role))
            .route(web::delete().to(r::role_delete)),
    )
    .route(
        "/ui/accounts/clusterroles/{name}/info/fragment",
        web::get().to(r::clusterrole_info_fragment),
    )
    .route(
        "/ui/accounts/clusterroles/{name}/rules/fragment",
        web::get().to(r::clusterrole_rules_fragment),
    )
    .service(
        web::resource("/ui/accounts/clusterroles/{name}")
            .route(web::get().to(r::clusterrole))
            .route(web::delete().to(r::clusterrole_delete)),
    )
    .route(
        "/ui/accounts/bindings/fragment",
        web::get().to(r::bindings_fragment),
    )
    .route("/ui/accounts/bindings", web::get().to(r::bindings))
    .route(
        "/ui/accounts/{namespace}/rolebindings/{name}/info/fragment",
        web::get().to(r::binding_info_fragment),
    )
    .route(
        "/ui/accounts/{namespace}/rolebindings/{name}/subjects/fragment",
        web::get().to(r::binding_subjects_fragment),
    )
    .service(
        web::resource("/ui/accounts/{namespace}/rolebindings/{name}")
            .route(web::get().to(r::binding))
            .route(web::delete().to(r::binding_delete)),
    )
    .route(
        "/ui/accounts/clusterrolebindings/{name}/info/fragment",
        web::get().to(r::clusterbinding_info_fragment),
    )
    .route(
        "/ui/accounts/clusterrolebindings/{name}/subjects/fragment",
        web::get().to(r::clusterbinding_subjects_fragment),
    )
    .service(
        web::resource("/ui/accounts/clusterrolebindings/{name}")
            .route(web::get().to(r::clusterbinding))
            .route(web::delete().to(r::clusterbinding_delete)),
    );
}

fn workloads_routes(cfg: &mut web::ServiceConfig) {
    use routes::workloads as r;

    cfg.route(
        "/ui/workloads/fragment",
        web::get().to(r::workloads_fragment),
    )
    .route(
        "/ui/workloads/stats/fragment",
        web::get().to(r::workloads_stats_fragment),
    )
    .route("/ui/workloads", web::get().to(r::workloads))
    .route(
        "/ui/workloads/configmaps/fragment",
        web::get().to(r::configmaps_fragment),
    )
    .route("/ui/workloads/configmaps", web::get().to(r::configmaps))
    .route(
        "/ui/workloads/{namespace}/configmaps/{name}/info/fragment",
        web::get().to(r::configmap_info_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/configmaps/{name}/data/fragment",
        web::get().to(r::configmap_data_fragment),
    )
    .service(
        web::resource("/ui/workloads/{namespace}/configmaps/{name}")
            .route(web::get().to(r::configmap))
            .route(web::delete().to(r::configmap_delete)),
    )
    .route(
        "/ui/workloads/services/fragment",
        web::get().to(r::services_fragment),
    )
    .route("/ui/workloads/services", web::get().to(r::services))
    .route(
        "/ui/workloads/{namespace}/services/{name}/info/fragment",
        web::get().to(r::service_info_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/services/{name}/events/fragment",
        web::get().to(r::service_events_fragment),
    )
    .service(
        web::resource("/ui/workloads/{namespace}/services/{name}")
            .route(web::get().to(r::service))
            .route(web::delete().to(r::service_delete)),
    )
    .route(
        "/ui/workloads/ingresses/fragment",
        web::get().to(r::ingresses_fragment),
    )
    .route("/ui/workloads/ingresses", web::get().to(r::ingresses))
    .route(
        "/ui/workloads/{namespace}/ingresses/{name}/info/fragment",
        web::get().to(r::ingress_info_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/ingresses/{name}/rules/fragment",
        web::get().to(r::ingress_rules_fragment),
    )
    .service(
        web::resource("/ui/workloads/{namespace}/ingresses/{name}")
            .route(web::get().to(r::ingress))
            .route(web::delete().to(r::ingress_delete)),
    )
    .route(
        "/ui/workloads/replicas/fragment",
        web::get().to(r::replicas_fragment),
    )
    .route("/ui/workloads/replicas", web::get().to(r::replicas))
    .route(
        "/ui/workloads/{namespace}/replicasets/{name}/stats/fragment",
        web::get().to(r::replica_stats_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/replicasets/{name}/info/fragment",
        web::get().to(r::replica_info_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/replicasets/{name}/container/fragment",
        web::get().to(r::replica_container_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/replicasets/{name}/pods/fragment",
        web::get().to(r::replica_pods_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/replicasets/{name}/events/fragment",
        web::get().to(r::replica_events_fragment),
    )
    .service(
        web::resource("/ui/workloads/{namespace}/replicasets/{name}")
            .route(web::get().to(r::replica))
            .route(web::delete().to(r::replica_delete)),
    )
    .route(
        "/ui/workloads/pods/fragment",
        web::get().to(r::pods_fragment),
    )
    .route("/ui/workloads/pods", web::get().to(r::pods))
    .route(
        "/ui/workloads/{namespace}/pods/{name}/stats/fragment",
        web::get().to(r::pod_stats_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/pods/{name}/info/fragment",
        web::get().to(r::pod_info_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/pods/{name}/container/fragment",
        web::get().to(r::pod_container_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/pods/{name}/events/fragment",
        web::get().to(r::pod_events_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/pods/{name}/logs/fragment",
        web::get().to(r::pod_logs_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/pods/{name}/logs/download",
        web::get().to(r::pod_logs_download),
    )
    .route(
        "/ui/workloads/{namespace}/pods/{name}/logs",
        web::get().to(r::pod_logs),
    )
    .route(
        "/ui/workloads/{namespace}/pods/{name}/exec",
        web::get().to(r::pod_exec),
    )
    .service(
        web::resource("/ui/workloads/{namespace}/pods/{name}")
            .route(web::get().to(r::pod))
            .route(web::delete().to(r::pod_delete)),
    )
    .route(
        "/ui/workloads/{namespace}/deployments/{name}/stats/fragment",
        web::get().to(r::deployment_stats_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/deployments/{name}/info/fragment",
        web::get().to(r::deployment_info_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/deployments/{name}/container/fragment",
        web::get().to(r::deployment_container_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/deployments/{name}/replicasets/fragment",
        web::get().to(r::deployment_replicasets_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/deployments/{name}/pods/fragment",
        web::get().to(r::deployment_pods_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/deployments/{name}/events/fragment",
        web::get().to(r::deployment_events_fragment),
    )
    .service(
        web::resource("/ui/workloads/{namespace}/deployments/{name}")
            .route(web::get().to(r::deployment))
            .route(web::delete().to(r::deployment_delete)),
    )
    .route(
        "/ui/workloads/{namespace}/jobs/{name}/stats/fragment",
        web::get().to(r::job_stats_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/jobs/{name}/info/fragment",
        web::get().to(r::job_info_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/jobs/{name}/container/fragment",
        web::get().to(r::job_container_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/jobs/{name}/pods/fragment",
        web::get().to(r::job_pods_fragment),
    )
    .route(
        "/ui/workloads/{namespace}/jobs/{name}/events/fragment",
        web::get().to(r::job_events_fragment),
    )
    .service(
        web::resource("/ui/workloads/{namespace}/jobs/{name}")
            .route(web::get().to(r::job))
            .route(web::delete().to(r::job_delete)),
    );
}

async fn dashboard(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(app::pages::dashboard::render(req.path()))
}

async fn profile(req: HttpRequest) -> HttpResponse {
    let claims = req
        .extensions()
        .get::<quench_auth::prelude::Claims>()
        .cloned();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(app::pages::profile::render(req.path(), claims.as_ref()))
}

async fn facade(req: HttpRequest, cache: web::Data<CacheStore>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(app::pages::facade::render(&cache, req.path()).await)
}

async fn facade_fragment(cache: web::Data<CacheStore>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(app::pages::facade::fragment(&cache).await.render())
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("Not found.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    quench_starter::actix::serve(root_scope, move || app_scope(auth.clone()), None, async {}).await
}
