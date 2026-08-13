use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use quench_auth::prelude::{Claims, JwtConfig};
use quench_cache::CacheStore;
use serde::Deserialize;

use super::{delete, html, NameFilter};

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

pub async fn workloads(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    query: web::Query<Filter>,
) -> HttpResponse {
    html(app::pages::workloads::render(&cache, req.path(), &query.namespace, &query.name).await)
}

pub async fn workloads_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<Filter>,
) -> HttpResponse {
    html(
        app::pages::workloads::fragment(&cache, &query.namespace, &query.name)
            .await
            .render(),
    )
}

pub async fn workloads_stats_fragment(
    cache: web::Data<CacheStore>,
    query: web::Query<NameFilter>,
) -> HttpResponse {
    html(
        app::pages::workloads::stats_fragment(&cache, &query.name)
            .await
            .render(),
    )
}

macro_rules! namespaced_list {
    ($render_fn:ident, $fragment_fn:ident, $module:path) => {
        pub async fn $render_fn(
            req: HttpRequest,
            cache: web::Data<CacheStore>,
            query: web::Query<Filter>,
        ) -> HttpResponse {
            use $module as m;
            html(m::render(&cache, req.path(), &query.namespace, &query.name).await)
        }

        pub async fn $fragment_fn(
            cache: web::Data<CacheStore>,
            query: web::Query<Filter>,
        ) -> HttpResponse {
            use $module as m;
            html(
                m::fragment(&cache, &query.namespace, &query.name)
                    .await
                    .render(),
            )
        }
    };
}

namespaced_list!(
    configmaps,
    configmaps_fragment,
    app::pages::workloads::configmaps
);
namespaced_list!(services, services_fragment, app::pages::workloads::services);
namespaced_list!(
    ingresses,
    ingresses_fragment,
    app::pages::workloads::ingresses
);
namespaced_list!(replicas, replicas_fragment, app::pages::workloads::replicas);
namespaced_list!(pods, pods_fragment, app::pages::workloads::pods);

pub async fn configmap(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::workloads::configmap::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn configmap_delete(
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
        "ConfigMap",
        Some(&namespace),
        &name,
        &format!("{}/workloads/configmaps", app::base_path::ui_base()),
    )
    .await
}

pub async fn configmap_info_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::workloads::configmap::info_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn configmap_data_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::workloads::configmap::data_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn service(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::workloads::service::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn service_delete(
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
        "Service",
        Some(&namespace),
        &name,
        &format!("{}/workloads/services", app::base_path::ui_base()),
    )
    .await
}

pub async fn service_info_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::workloads::service::info_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn service_events_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    let events_url = format!(
        "{}/workloads/{namespace}/services/{name}/events/fragment",
        app::base_path::ui_base()
    );
    html(
        app::components::events::render(&cache, "Service", &namespace, &name, &events_url)
            .await
            .render(),
    )
}

pub async fn ingress(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::workloads::ingress::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn ingress_delete(
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
        "Ingress",
        Some(&namespace),
        &name,
        &format!("{}/workloads/ingresses", app::base_path::ui_base()),
    )
    .await
}

pub async fn ingress_info_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::workloads::ingress::info_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn ingress_rules_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::workloads::ingress::rules_fragment(&cache, &namespace, &name)
            .await
            .render(),
    )
}

pub async fn deployment(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::workloads::deployment::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn deployment_delete(
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
        "Deployment",
        Some(&namespace),
        &name,
        &format!("{}/workloads", app::base_path::ui_base()),
    )
    .await
}

macro_rules! deployment_fragment {
    ($name:ident, $fn:ident) => {
        pub async fn $name(
            cache: web::Data<CacheStore>,
            path: web::Path<(String, String)>,
        ) -> HttpResponse {
            let (namespace, name) = path.into_inner();
            html(
                app::pages::workloads::deployment::$fn(&cache, &namespace, &name)
                    .await
                    .render(),
            )
        }
    };
}

deployment_fragment!(deployment_stats_fragment, stats_fragment);
deployment_fragment!(deployment_info_fragment, info_fragment);
deployment_fragment!(deployment_container_fragment, container_fragment);
deployment_fragment!(deployment_replicasets_fragment, replicasets_fragment);
deployment_fragment!(deployment_pods_fragment, pods_fragment);

pub async fn deployment_events_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    let events_url = format!(
        "{}/workloads/{namespace}/deployments/{name}/events/fragment",
        app::base_path::ui_base()
    );
    html(
        app::components::events::render(&cache, "Deployment", &namespace, &name, &events_url)
            .await
            .render(),
    )
}

pub async fn job(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::workloads::job::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn job_delete(
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
        "Job",
        Some(&namespace),
        &name,
        &format!("{}/workloads", app::base_path::ui_base()),
    )
    .await
}

macro_rules! job_fragment {
    ($name:ident, $fn:ident) => {
        pub async fn $name(
            cache: web::Data<CacheStore>,
            path: web::Path<(String, String)>,
        ) -> HttpResponse {
            let (namespace, name) = path.into_inner();
            html(
                app::pages::workloads::job::$fn(&cache, &namespace, &name)
                    .await
                    .render(),
            )
        }
    };
}

job_fragment!(job_stats_fragment, stats_fragment);
job_fragment!(job_info_fragment, info_fragment);
job_fragment!(job_container_fragment, container_fragment);
job_fragment!(job_pods_fragment, pods_fragment);

pub async fn job_events_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    let events_url = format!(
        "{}/workloads/{namespace}/jobs/{name}/events/fragment",
        app::base_path::ui_base()
    );
    html(
        app::components::events::render(&cache, "Job", &namespace, &name, &events_url)
            .await
            .render(),
    )
}

pub async fn replica(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::workloads::replica::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn replica_delete(
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
        "ReplicaSet",
        Some(&namespace),
        &name,
        &format!("{}/workloads/replicas", app::base_path::ui_base()),
    )
    .await
}

macro_rules! replica_fragment {
    ($name:ident, $fn:ident) => {
        pub async fn $name(
            cache: web::Data<CacheStore>,
            path: web::Path<(String, String)>,
        ) -> HttpResponse {
            let (namespace, name) = path.into_inner();
            html(
                app::pages::workloads::replica::$fn(&cache, &namespace, &name)
                    .await
                    .render(),
            )
        }
    };
}

replica_fragment!(replica_stats_fragment, stats_fragment);
replica_fragment!(replica_info_fragment, info_fragment);
replica_fragment!(replica_container_fragment, container_fragment);
replica_fragment!(replica_pods_fragment, pods_fragment);

pub async fn replica_events_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    let events_url = format!(
        "{}/workloads/{namespace}/replicasets/{name}/events/fragment",
        app::base_path::ui_base()
    );
    html(
        app::components::events::render(&cache, "ReplicaSet", &namespace, &name, &events_url)
            .await
            .render(),
    )
}

pub async fn pod(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(app::pages::workloads::pod::render(&cache, req.path(), &namespace, &name).await)
}

pub async fn pod_delete(
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
        "Pod",
        Some(&namespace),
        &name,
        &format!("{}/workloads/pods", app::base_path::ui_base()),
    )
    .await
}

macro_rules! pod_fragment {
    ($name:ident, $fn:ident) => {
        pub async fn $name(
            cache: web::Data<CacheStore>,
            path: web::Path<(String, String)>,
        ) -> HttpResponse {
            let (namespace, name) = path.into_inner();
            html(
                app::pages::workloads::pod::$fn(&cache, &namespace, &name)
                    .await
                    .render(),
            )
        }
    };
}

pod_fragment!(pod_stats_fragment, stats_fragment);
pod_fragment!(pod_info_fragment, info_fragment);
pod_fragment!(pod_container_fragment, container_fragment);

pub async fn pod_events_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    let events_url = format!(
        "{}/workloads/{namespace}/pods/{name}/events/fragment",
        app::base_path::ui_base()
    );
    html(
        app::components::events::render(&cache, "Pod", &namespace, &name, &events_url)
            .await
            .render(),
    )
}

#[derive(Deserialize)]
pub struct LogsQuery {
    #[serde(default)]
    container: String,
    /// Present (any value, incl. empty) when the checkbox was checked -
    /// absent otherwise, since unchecked HTML checkboxes contribute nothing
    /// to the submitted params at all.
    #[serde(default)]
    previous: Option<String>,
    #[serde(default)]
    name: String,
}

impl LogsQuery {
    fn previous(&self) -> bool {
        self.previous.is_some()
    }
}

pub async fn pod_logs(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
    query: web::Query<LogsQuery>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::workloads::pod_logs::render(
            &cache,
            req.path(),
            &namespace,
            &name,
            &query.container,
            query.previous(),
            &query.name,
        )
        .await,
    )
}

pub async fn pod_logs_fragment(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
    query: web::Query<LogsQuery>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    html(
        app::pages::workloads::pod_logs::fragment(
            &cache,
            &namespace,
            &name,
            &query.container,
            query.previous(),
            &query.name,
        )
        .await
        .render(),
    )
}

#[derive(Deserialize)]
pub struct LogsDownloadQuery {
    #[serde(default)]
    container: String,
    #[serde(default)]
    previous: bool,
}

pub async fn pod_logs_download(
    cache: web::Data<CacheStore>,
    path: web::Path<(String, String)>,
    query: web::Query<LogsDownloadQuery>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    let logs = api::resource::logs(
        &cache,
        "Pod",
        namespace,
        name.clone(),
        query.container.clone(),
        query.previous,
        -1,
    )
    .await
    .unwrap_or_default();

    let now = chrono::Local::now().naive_local();
    let filename = format!("logs_{name}_{}.log", now.format("%Y-%m-%d_%H-%M-%S-%3f"));

    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"{filename}\""),
        ))
        .body(logs.join("\n"))
}

#[derive(Deserialize)]
pub struct ExecQuery {
    #[serde(default)]
    container: String,
}

pub async fn pod_exec(
    req: HttpRequest,
    cache: web::Data<CacheStore>,
    config: web::Data<JwtConfig>,
    path: web::Path<(String, String)>,
    query: web::Query<ExecQuery>,
) -> HttpResponse {
    let (namespace, name) = path.into_inner();
    let claims = req.extensions().get::<Claims>().cloned();
    html(
        app::pages::workloads::pod_exec::render(
            &cache,
            &config,
            claims.as_ref(),
            req.path(),
            &namespace,
            &name,
            &query.container,
        )
        .await,
    )
}
