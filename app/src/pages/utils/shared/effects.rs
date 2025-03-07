use std::sync::{Arc, Mutex};
use std::time::Duration;
use leptos::prelude::*;

pub fn update_page_effect<F>(timeout: u32, callback: F) -> Arc<Mutex<Option<IntervalHandle>>>
where
    F: Fn() + Clone + 'static
{
    let interval_handle = Arc::new(Mutex::new(None));
    let interval_handle_clone = Arc::clone(&interval_handle);

    Effect::new(move |_| {
        let callback = callback.clone();
        callback();
        let id = set_interval_with_handle(move || {
            callback();
        }, Duration::from_millis(timeout.into()));

        if let Ok(id) = id {
            *interval_handle_clone.lock().unwrap() = Some(id);
        }
    });
    interval_handle
}

pub fn clear_page_effect(interval_handle: Arc<Mutex<Option<IntervalHandle>>>) {
    on_cleanup(move ||
        if let Some(handle) = interval_handle.lock().unwrap().as_ref() {
            handle.clear()
        });
}
