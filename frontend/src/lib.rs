#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // `#[server(..., "/api/...")]` paths are absolute, so the browser's
    // `fetch` resolves them against the origin root, not this page's own
    // `BASE_PATH`-prefixed URL - exactly the same blind spot `<Router base>`
    // has, but for a completely different mechanism (server_fn's own client,
    // not leptos_router). `set_server_url` is server_fn's own sanctioned fix
    // for a subpath deployment; it needs a `'static str`, and `BASE_PATH`
    // only exists at runtime, so it's leaked once here rather than read on
    // every call - this runs exactly once per page load.
    let base_path: &'static str = Box::leak(base_path::base_path().into_boxed_str());
    leptos::server_fn::client::set_server_url(base_path);

    leptos::mount::hydrate_body(web_app::WebApp);
}
