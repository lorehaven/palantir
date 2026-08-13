//! Palantir's page chrome.
//!
//! quench-web's own top bar (title, logout, a hamburger trigger), a
//! breadcrumb, and a footer are static, shared chrome built once through
//! [`quench_web::AppShell`]. The slide-out drawer that trigger opens -
//! palantir's own resource-navigation entries, then quench-web's
//! theme/locale switcher underneath - is *not* part of that static shell:
//! which entries are highlighted depends on the current page, so it's built
//! fresh per request in [`page`] instead, same as the breadcrumb is.
//!
//! quench-web ships no sidebar of its own to move that navigation list into:
//! its `NavPanelBuilder` only ever renders the two `<select>`s, with no
//! extension point for arbitrary entries. So this hand-rolls the same
//! `modal-overlay`/`modal-side`/`modal-content` structure `NavPanelBuilder`
//! and `nav_button()`'s `toggle_modal("modal-overlay", "modal-side", "show")`
//! expect, and puts the entries in ahead of the selects. This also sidesteps
//! a `NavPanelBuilder::build()` bug: it emits its own locale- and
//! theme-select `on_dom_ready` blocks joined into *one* function body, both
//! declaring `const select`, which is an immediate
//! `SyntaxError: Identifier 'select' has already been declared`. Each
//! select's init script stays in its own `<script>` tag (own scope) here.

use std::sync::OnceLock;

use quench_web::nav_button;
use quench_web::prelude::*;

static SHELL: OnceLock<AppShell> = OnceLock::new();

/// Every theme quench-web ships. Palantir's own component styling
/// (`styles/*.scss`) is hardcoded to a dark palette and doesn't move with
/// this, but quench-web's own chrome - the header, the footer, the
/// theme/locale panel itself - is built on its `--q-shell-*`/`--bs-*` custom
/// properties and does.
fn supported_themes() -> Vec<Theme> {
    vec![
        Theme::DefaultDark,
        Theme::DefaultLight,
        Theme::BootstrapDark,
        Theme::BootstrapLight,
    ]
}

/// The only locale with real translations (`i18n/en-US.ftl`) - without a
/// file there for it, `available_locales()` finds nothing and the panel's
/// locale `<select>` renders with zero `<option>`s.
fn supported_locales() -> Vec<String> {
    vec!["en-US".to_string()]
}

fn shell() -> &'static AppShell {
    SHELL.get_or_init(|| {
        let resources_prefix = crate::base_path::base_path();
        AppShellBuilder::new()
            .title("Palantir")
            .default_theme(Theme::DefaultDark)
            .supported_themes(supported_themes())
            .supported_locales(supported_locales())
            .header(top_header())
            .footer(app_footer())
            // Palantir's own hand-written styling (`styles/*.scss`, compiled
            // to `dist/assets/css/palantir.css` by `run.sh`/the Dockerfile -
            // see those for the actual `sass` invocation) - loaded from the
            // same `/assets` mount quench-web's own generated CSS/JS use,
            // rather than anything cargo-leptos used to produce.
            .links(vec![Link::new(
                "stylesheet",
                &format!("{resources_prefix}/assets/css/palantir.css"),
            )])
            .resources_prefix(resources_prefix)
            .build()
    })
}

/// Top bar built the same way every other quench-web service in the estate
/// builds theirs (see e.g. gatehouse-service's `ui_header`): a raw `header()`
/// with a `left-panel`/`right-panel` pair rather than `HeaderBuilder`, which
/// has no slot for anything past the nav trigger and title. quench-web's own
/// shared CSS (`dist/assets/css/style.css`, `header`/`.q-shell-header`) lays
/// left/right panels out via `justify-content: space-between` and already
/// styles the nav trigger, so this needs no palantir-specific CSS of its own
/// beyond the logout link. `nav_button()` opens the drawer `page()` renders
/// per request - see the module doc for why that isn't built here instead.
fn top_header() -> Element {
    header()
        .child(
            div()
                .class("left-panel")
                .child(nav_button())
                .child(h2().text("Palantir")),
        )
        .child(
            div().class("right-panel").child(
                a().attr("href", format!("{}/logout", crate::base_path::ui_base()))
                    .class("button")
                    .text("Logout"),
            ),
        )
}

/// Wraps `content` in palantir's chrome and renders the full HTML document.
///
/// `breadcrumbs` mirrors the old `<Header text=.../>` component: `"Palantir"`
/// always leads, every segment up to the last links to the path built from
/// the segments before it, and the last segment is plain text.
/// `current_path` drives which nav-drawer section is highlighted.
pub fn page(breadcrumbs: &[&str], current_path: &str, content: Element) -> String {
    shell().page(
        div()
            .class("content")
            .child(nav::panel(current_path))
            .child(nav::apply_dialog())
            .child(
                div()
                    .class("content-internal")
                    .child(header::breadcrumb(breadcrumbs))
                    .child(content),
            ),
    )
}

fn app_footer() -> Element {
    quench_web::footer()
        .class("footer")
        .text("© 2025 Paweł Walus - Order of Devs | LoreHaven")
}

mod header {
    use quench_web::prelude::*;

    pub fn breadcrumb(segments: &[&str]) -> Element {
        let base = crate::base_path::ui_base();
        let mut el = div().class("header").child(
            a().attr("href", base.clone())
                .class("header-link")
                .text("Palantir"),
        );

        let mut path_parts: Vec<String> = Vec::new();
        for (idx, segment) in segments.iter().enumerate() {
            path_parts.push(segment.to_lowercase());
            let is_last = idx == segments.len() - 1;
            el = el.child(span().class("header-separator").text(" / "));
            el = el.child(if is_last {
                span().class("header-link").text(*segment)
            } else {
                let href = format!("{base}/{}", path_parts.join("/"));
                a().attr("href", href).class("header-link").text(*segment)
            });
        }
        el
    }
}

mod nav {
    use quench_web::framework::dom::{
        on_dom_ready, set_select_value, toggle_modal, update_from_select,
    };
    use quench_web::prelude::*;

    use crate::components::shared::dialog::apply_yaml::apply_yaml_dialog;

    const APPLY_OVERLAY: &str = "apply-yaml-overlay";
    const APPLY_PANEL: &str = "apply-yaml-panel";

    struct Entry {
        name: &'static str,
        icon: &'static str,
        icon_type: &'static str,
        url_prefix: &'static str,
        /// Section prefix this entry is only shown under (e.g. `/cluster`
        /// entries only show while browsing somewhere under `/cluster`).
        /// Top-level section entries (cluster, workloads, ...) pass `""`,
        /// which is always visible.
        section: &'static str,
    }

    const ENTRIES: &[Entry] = &[
        Entry {
            name: "cluster",
            icon: "cube",
            icon_type: "solid",
            url_prefix: "",
            section: "",
        },
        Entry {
            name: "nodes",
            icon: "hexagon-nodes",
            icon_type: "solid",
            url_prefix: "/cluster",
            section: "/cluster",
        },
        Entry {
            name: "namespaces",
            icon: "circle-nodes",
            icon_type: "solid",
            url_prefix: "/cluster",
            section: "/cluster",
        },
        Entry {
            name: "workloads",
            icon: "rotate-right",
            icon_type: "solid",
            url_prefix: "",
            section: "",
        },
        Entry {
            name: "services",
            icon: "network-wired",
            icon_type: "solid",
            url_prefix: "/workloads",
            section: "/workloads",
        },
        Entry {
            name: "replicas",
            icon: "object-ungroup",
            icon_type: "solid",
            url_prefix: "/workloads",
            section: "/workloads",
        },
        Entry {
            name: "pods",
            icon: "boxes-stacked",
            icon_type: "solid",
            url_prefix: "/workloads",
            section: "/workloads",
        },
        Entry {
            name: "ingresses",
            icon: "circle-nodes",
            icon_type: "solid",
            url_prefix: "/workloads",
            section: "/workloads",
        },
        Entry {
            name: "configmaps",
            icon: "sliders",
            icon_type: "solid",
            url_prefix: "/workloads",
            section: "/workloads",
        },
        Entry {
            name: "storage",
            icon: "warehouse",
            icon_type: "solid",
            url_prefix: "",
            section: "",
        },
        Entry {
            name: "volumes",
            icon: "hard-drive",
            icon_type: "solid",
            url_prefix: "/storage",
            section: "/storage",
        },
        Entry {
            name: "claims",
            icon: "hard-drive",
            icon_type: "regular",
            url_prefix: "/storage",
            section: "/storage",
        },
        Entry {
            name: "accounts",
            icon: "users",
            icon_type: "solid",
            url_prefix: "",
            section: "",
        },
        Entry {
            name: "roles",
            icon: "user-lock",
            icon_type: "solid",
            url_prefix: "/accounts",
            section: "/accounts",
        },
        Entry {
            name: "bindings",
            icon: "user-lock",
            icon_type: "solid",
            url_prefix: "/accounts",
            section: "/accounts",
        },
        Entry {
            name: "secrets",
            icon: "lock",
            icon_type: "solid",
            url_prefix: "/accounts",
            section: "/accounts",
        },
        Entry {
            name: "profile",
            icon: "user",
            icon_type: "solid",
            url_prefix: "",
            section: "",
        },
    ];

    /// Sections that get a visual separator after their last entry, in
    /// `ENTRIES` order (cluster's two children, workloads' five, storage's
    /// two, accounts' three, then profile stands alone before "apply").
    const SEPARATOR_AFTER: &[&str] = &["namespaces", "configmaps", "claims", "secrets", "profile"];

    /// The drawer `top_header`'s `nav_button()` opens: palantir's own
    /// resource-navigation entries and the create/apply action, then
    /// quench-web's theme/locale switcher underneath. Rebuilt per request
    /// (unlike the static header) purely so `current_path` can drive which
    /// entry is highlighted - see the module doc for the rest.
    pub fn panel(current_path: &str) -> Element {
        let mut entries = div().class("side-nav-bar");
        for entry in ENTRIES {
            // `current_path` is the full request path (BASE_PATH + `/ui` +
            // ...), so a section match has to look for the segment anywhere
            // in it rather than assuming it's a prefix.
            let visible = entry.section.is_empty() || current_path.contains(entry.section);
            entries = entries.child(entry_element(entry, visible));
            if SEPARATOR_AFTER.contains(&entry.name) {
                entries = entries.child(div().class("separator"));
            }
        }
        entries = entries.child(apply_entry());

        let toggle = toggle_modal("modal-overlay", "modal-side", "show");
        div()
            .child(div().class("modal-overlay").on_click(&toggle))
            .child(
                div().class("modal-side").child(
                    div()
                        .class("modal-content")
                        .child(entries)
                        .child(div().class("separator"))
                        .child(label().attr("data-i18n", "locale_label"))
                        .child(locale_switch(Some(crate::shell::supported_locales()), None))
                        .child(label().attr("data-i18n", "theme_label"))
                        .child(theme_select()),
                ),
            )
            .child(
                script(on_dom_ready(&[set_select_value(
                    "theme-select",
                    "getTheme",
                )]))
                .raw()
                .defer(),
            )
    }

    fn theme_select() -> Element {
        crate::shell::supported_themes().into_iter().fold(
            select()
                .attr("id", "theme-select")
                .attr("value", Theme::DefaultDark.to_string())
                .on_change(update_from_select("theme-select", "updateTheme")),
            |el, theme| {
                let theme_str = theme.to_string();
                el.child(option().attr("value", &theme_str).text(&theme_str))
            },
        )
    }

    fn entry_element(entry: &Entry, visible: bool) -> Element {
        let base = crate::base_path::ui_base();
        let class = format!(
            "side-nav-bar-entry {}",
            if visible { "visible" } else { "hidden" }
        );
        let icon_class = format!("fa-{} fa-{}", entry.icon_type, entry.icon);
        let title = capitalize(entry.name);
        let href = format!("{base}{}/{}", entry.url_prefix, entry.name);

        a().attr("href", href)
            .class(class)
            .child(i().class(icon_class))
            .child(div().class("side-nav-bar-entry-title").text(title))
    }

    /// Opens `apply_dialog()` rather than navigating.
    fn apply_entry() -> Element {
        div()
            .class("side-nav-bar-entry visible")
            .attr("onclick", toggle_modal(APPLY_OVERLAY, APPLY_PANEL, "show"))
            .child(i().class("fa-solid fa-plus"))
            .child(div().class("side-nav-bar-entry-title").text("Apply"))
    }

    /// Create-a-new-resource dialog, reachable from every page via the
    /// nav-drawer "+" entry above - an empty-textarea, `POST`-mode instance
    /// of the same dialog `actions::edit::edit_action` shows pre-filled for
    /// an existing resource.
    pub fn apply_dialog() -> Element {
        let submit_url = format!("{}/apply", crate::base_path::ui_base());
        apply_yaml_dialog(APPLY_OVERLAY, APPLY_PANEL, "", "post", &submit_url)
    }

    fn capitalize(value: &str) -> String {
        let mut chars = value.chars();
        chars.next().map_or_else(String::new, |first| {
            first.to_uppercase().collect::<String>() + chars.as_str()
        })
    }
}
