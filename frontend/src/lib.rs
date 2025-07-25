#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(web_app::WebApp);
}
