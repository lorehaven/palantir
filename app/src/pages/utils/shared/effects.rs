use leptos::prelude::*;
use wasm_bindgen::prelude::*;

pub fn update_page_effect<F>(timeout: i32, callback: F)
where F: Fn() + Clone + 'static{
    Effect::new(move |_| {
        let callback = callback.clone();
        callback();

        let interval_callback = Closure::wrap(Box::new(move || {
            callback();
        }) as Box<dyn FnMut()>);
        let interval_callback_ref = interval_callback.as_ref().unchecked_ref::<js_sys::Function>();

        web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(interval_callback_ref, timeout)
            .unwrap();

        move || {
            interval_callback.forget();
        }
    });
}
