use app::*;
use axum::Router;
use axum::routing::get;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

pub mod ws;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(web_app::WebApp);

    let app = Router::new()
        .route("/ws/exec", get(ws::exec_ws_handler))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell::shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell::shell))
        .with_state(leptos_options);

    leptos::logging::log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
