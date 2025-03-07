use std::time::Duration;
use leptos::prelude::*;

pub fn update_page_effect<F>(timeout: u32, callback: F)
where
    F: Fn() + Clone + 'static
{
    Effect::new(move |_| {
        let callback = callback.clone();
        callback();
        let _ = set_interval(move || {
            callback();
        }, Duration::from_millis(timeout.into()));
    });
}
