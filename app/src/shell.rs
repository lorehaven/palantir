//! Palantir's page chrome: breadcrumb header, side nav, and footer, wrapped
//! around each page's own content and rendered through a shared
//! [`quench_web::AppShell`].
//!
//! The shell's default header/nav (`AppShellBuilder`'s theme+locale slide-out
//! panel) doesn't fit palantir's own persistent sidebar layout, so it's built
//! with `.with_header(false)` and the breadcrumb/sidebar/footer are composed
//! into the page content instead - the same shape the old Leptos
//! `PageContent` component had.

use std::sync::OnceLock;

use quench_web::prelude::*;

static SHELL: OnceLock<AppShell> = OnceLock::new();

fn shell() -> &'static AppShell {
    SHELL.get_or_init(|| {
        let resources_prefix = crate::base_path::base_path();
        AppShellBuilder::new()
            .title("Palantir")
            .default_theme(Theme::DefaultDark)
            .with_header(false)
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

/// Wraps `content` in palantir's chrome and renders the full HTML document.
///
/// `breadcrumbs` mirrors the old `<Header text=.../>` component: `"Palantir"`
/// always leads, every segment up to the last links to the path built from
/// the segments before it, and the last segment is plain text.
/// `current_path` drives which side-nav section is highlighted.
pub fn page(breadcrumbs: &[&str], current_path: &str, content: Element) -> String {
    shell().page(
        div()
            .class("content")
            .child(nav::side_nav_bar(current_path))
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
    use quench_web::framework::dom::toggle_modal;
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

    pub fn side_nav_bar(current_path: &str) -> Element {
        let mut bar = div().class("side-nav-bar");
        for entry in ENTRIES {
            // `current_path` is the full request path (BASE_PATH + `/ui` +
            // ...), so a section match has to look for the segment anywhere
            // in it rather than assuming it's a prefix.
            let visible = entry.section.is_empty() || current_path.contains(entry.section);
            bar = bar.child(entry_element(entry, visible));
            if SEPARATOR_AFTER.contains(&entry.name) {
                bar = bar.child(div().class("separator"));
            }
        }
        bar.child(apply_entry())
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
    /// side-nav "+" entry above - an empty-textarea, `POST`-mode instance of
    /// the same dialog `actions::edit::edit_action` shows pre-filled for an
    /// existing resource.
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
