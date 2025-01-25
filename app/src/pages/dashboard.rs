use leptos::prelude::*;
use leptos::task::spawn;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let nodes = RwSignal::new(vec![]);
    let pods = RwSignal::new(vec![]);
    let nodes_str = RwSignal::new(String::new());
    let pods_str = RwSignal::new(String::new());
    let loading = RwSignal::new(true);

    Effect::new(move |_| spawn(async move {
        nodes.set(crate::api::metrics::get_nodes().await.unwrap_or_default());
        pods.set(crate::api::metrics::get_pods().await.unwrap_or_default());
        nodes_str.set(format!("{:?}", nodes.get()));
        pods_str.set(format!("{:?}", pods.get()));

        loading.set(false);
    }));

    view! {
        <div>Dashboard</div>
        <div>{nodes_str}</div>
        <div>{pods_str}</div>
    }
}
