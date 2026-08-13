use quench_auth::prelude::Claims;
use quench_web::prelude::*;

/// The server already has the caller's decoded `Claims` (put there by
/// `auth_gate`) - unlike the old Leptos page, there's no need to fetch the
/// raw token and re-decode it client-side.
pub fn render(current_path: &str, claims: Option<&Claims>) -> String {
    let token = claims
        .and_then(|c| serde_json::to_string_pretty(c).ok())
        .unwrap_or_else(|| "not authenticated".to_string());

    crate::shell::page(
        &["Account", "Token"],
        current_path,
        div()
            .class("profile main-page")
            .child(div().class("profile-title").text("Current user"))
            .child(
                div().class("card-container dcc-1").child(
                    div().class("card-list").child(
                        div()
                            .class("card-list-row")
                            .child(div().class("card-list-row-title").text("Token"))
                            .child(
                                div()
                                    .class("card-list-row-content")
                                    .child(pre().text(token)),
                            ),
                    ),
                ),
            ),
    )
}
