use leptos::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Error,
    #[default]
    Info,
}

#[derive(Debug, Clone, Default)]
pub struct Toast {
    id: String,
    message: String,
    toast_type: ToastType,
}

#[derive(Debug, Clone, Default)]
pub struct ToasterContext {
    toasts: RwSignal<Vec<Toast>>,
}

impl ToasterContext {
    pub fn new() -> Self {
        Self {
            toasts: RwSignal::new(Vec::new()),
        }
    }

    pub fn add_toast(&self, message: impl Into<String>, toast_type: ToastType) {
        let id = window()
            .performance()
            .expect("performance should be available")
            .now()
            .to_string();
        let toast = Toast {
            id: id.clone(),
            message: message.into(),
            toast_type,
        };
        self.toasts.update(|toasts| toasts.push(toast));

        let toasts = self.toasts;
        set_timeout(
            move || toasts.update(|ts| ts.retain(|t| t.id != id)),
            std::time::Duration::from_secs(3),
        );
    }

    pub fn success(&self, message: impl Into<String>) {
        self.add_toast(message, ToastType::Success);
    }

    pub fn error(&self, message: impl Into<String>) {
        self.add_toast(message, ToastType::Error);
    }

    pub fn info(&self, message: impl Into<String>) {
        self.add_toast(message, ToastType::Info);
    }

    pub fn clear(&self) {
        self.toasts.set(Vec::new());
    }
}

pub fn provide_toaster() {
    provide_context(ToasterContext::new());
}

pub fn expect_toaster() -> ToasterContext {
    expect_context::<ToasterContext>()
}

#[component]
pub fn Toaster() -> impl IntoView {
    let toaster = expect_toaster();
    let toasts = toaster.toasts;

    view! {
        <div class="toaster-container">
            {move || toasts.get().into_iter().map(|toast| {
                let toast_class = match toast.toast_type {
                    ToastType::Success => "toast toast-success",
                    ToastType::Error => "toast toast-error",
                    ToastType::Info => "toast toast-info",
                };
                view! {
                    <div class=toast_class>
                        <span>{toast.message}</span>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
