//! `BASE_PATH` is a per-deployment runtime env var, so the Leptos
//! `<Router>`'s own `base` has to be resolved dynamically too, on both sides
//! of hydration.
//!
//! The server reads its own env var directly. The wasm/hydrate bundle has no
//! process env to read - `std::env::var` always misses there - so `shell.rs`
//! writes the value into a `window` global while rendering the page, before
//! the client bundle even starts downloading, and this module reads it back.
//! Skipping this - e.g. leaving the client to assume a bare `/ui` - is what
//! silently broke every route once `BASE_PATH` stopped being `/`: SSR route
//! matching strips the *full* request path (`req.path()`, `BASE_PATH` and
//! all) against `<Router base>`, so a base that omits `BASE_PATH` never
//! matches anything and falls through to the router's "not found" view.
//!
//! `WebApp` itself runs server-side in two different contexts that need two
//! different answers from this module: `leptos_actix::generate_route_list`
//! walks it once at boot, with no request in play, to build the literal
//! strings actix registers as routes - and `quench_starter::actix::serve()`
//! already nests that whole scope under `BASE_PATH` on its own, so those
//! strings must stay relative (`/ui`) or the real prefix gets doubled and
//! every route 404s. Per-request SSR is the opposite: it matches against the
//! *raw* incoming path (`BASE_PATH` and all), so it needs the full prefix.
//! `during_route_enumeration` is how `main.rs` tells this module which of
//! the two is currently running.

use std::sync::atomic::{AtomicBool, Ordering};

/// Set only around the single, synchronous `generate_route_list` call at
/// boot - see this module's doc comment.
static ENUMERATING_ROUTES: AtomicBool = AtomicBool::new(false);

#[cfg(not(target_arch = "wasm32"))]
pub fn during_route_enumeration<T>(f: impl FnOnce() -> T) -> T {
    ENUMERATING_ROUTES.store(true, Ordering::SeqCst);
    let result = f();
    ENUMERATING_ROUTES.store(false, Ordering::SeqCst);
    result
}

#[cfg(not(target_arch = "wasm32"))]
pub fn base_path() -> String {
    if ENUMERATING_ROUTES.load(Ordering::SeqCst) {
        return String::new();
    }
    normalize(&std::env::var("BASE_PATH").unwrap_or_default())
}

#[cfg(target_arch = "wasm32")]
pub fn base_path() -> String {
    let raw = web_sys::window()
        .and_then(|window| {
            js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("__BASE_PATH__")).ok()
        })
        .and_then(|value| value.as_string())
        .unwrap_or_default();
    normalize(&raw)
}

fn normalize(raw: &str) -> String {
    let trimmed = raw.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        String::new()
    } else if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

/// The Leptos `<Router>`'s own base: `BASE_PATH` (if any) plus the app's
/// `/ui` mount point, matching how `quench_starter::actix::serve()` nests
/// this app's scope under `BASE_PATH` on the actix side.
pub fn router_base() -> String {
    format!("{}/ui", base_path())
}
