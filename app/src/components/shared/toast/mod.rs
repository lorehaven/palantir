// use leptoaster::{expect_toaster, ToastBuilder, ToastLevel, ToastPosition};

#[derive(Debug, Clone, Default)]
pub struct Toast {}

impl Toast {
    pub fn success(message: &str) {
        leptos::logging::log!("{message}");
        // base_toast(message, ToastLevel::Success);
    }

    pub fn info(message: &str) {
        leptos::logging::log!("{message}");
        // base_toast(message, ToastLevel::Info);
    }

    pub fn warn(message: &str) {
        leptos::logging::log!("{message}");
        // base_toast(message, ToastLevel::Warn);
    }

    pub fn error(message: &str) {
        leptos::logging::log!("{message}");
        // base_toast(message, ToastLevel::Error);
    }
}

// fn base_toast(message: &str, level: ToastLevel) {
//     expect_toaster().toast(ToastBuilder::new(message)
//         .with_level(level)
//         .with_dismissable(true)
//         .with_expiry(Some(3_000))
//         .with_progress(true)
//         .with_position(ToastPosition::TopRight))
// }
