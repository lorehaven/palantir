use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::base_path::base_path;
use crate::web_app::WebApp;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    // The wasm bundle has no process env to read `BASE_PATH` from itself, so
    // it's handed over here, before `<HydrationScripts>` starts loading that
    // bundle - see `base_path`'s own doc comment for why this exists at all.
    let base_path_script = format!(
        "window.__BASE_PATH__={};",
        serde_json::to_string(&base_path()).unwrap_or_else(|_| "\"\"".to_string())
    );

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href=format!("{}/favicon.ico", base_path())/>
                <script inner_html=base_path_script></script>
                <AutoReload options=options.clone()/>
                <HydrationScripts options root=base_path()/>
                <MetaTags/>
            </head>
            <body>
                <WebApp/>
            </body>
        </html>
    }
}
