use leptos::prelude::*;
use leptos::task::spawn;

use crate::pages::utils::dashboard::*;
use crate::pages::components::prelude::*;
use crate::pages::utils::time::time_until_now;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let nodes_ready = RwSignal::new((0., 0.));
    let nodes_cpu = RwSignal::new((0., 0.));
    let nodes_memory_values = RwSignal::new((0., 0.));
    let nodes_memory_labels = RwSignal::new((String::new(), String::new()));

    let pods_ready = RwSignal::new((0., 0.));
    let pods_cpu = RwSignal::new((0., 0.));
    let pods_memory_values = RwSignal::new((0., 0.));
    let pods_memory_labels = RwSignal::new((String::new(), String::new()));

    let events = RwSignal::new(vec![]);

    let loading = RwSignal::new(true);

    Effect::new(move |_| spawn(async move {
        let nodes = crate::api::nodes::get_nodes().await.unwrap_or_default();
        let nodes_metrics = crate::api::metrics::get_nodes().await.unwrap_or_default();
        nodes_ready.set(get_nodes_ready(&nodes));
        nodes_cpu.set(get_nodes_cpu(&nodes, &nodes_metrics));
        let nodes_memory = get_nodes_memory(&nodes, &nodes_metrics);
        nodes_memory_values.set(nodes_memory.0);
        nodes_memory_labels.set(nodes_memory.1);

        let pods = crate::api::pods::get_pods().await.unwrap_or_default();
        let pods_metrics = crate::api::metrics::get_pods().await.unwrap_or_default();
        pods_ready.set(get_pods_ready(&pods));
        pods_cpu.set(get_pods_cpu(&pods, &pods_metrics));
        let pods_memory = get_pods_memory(&pods, &pods_metrics);
        pods_memory_values.set(pods_memory.0);
        pods_memory_labels.set(pods_memory.1);

        let events_list = crate::api::events::get_events().await.unwrap_or_default();
        events.set(events_list.into_iter().map(|e|
            vec![e.involved_object.kind, e.involved_object.name, time_until_now(&e.first_timestamp), e.reason, e.message]
        ).collect());

        loading.set(false);
    }));

    view! {
        <Header text=" > Overview".to_string() />
        <PageContent>
            <PageContentSlot slot>
                <div class="dashboard">
                    <Expandable label="Nodes" expanded=true>
                        <ExpandableSlot slot>
                            <div class="dashboard-card-container dcc-3">
                                <DashboardCardCircle
                                    label="Nodes"
                                    label_add="ready vs all"
                                    values=nodes_ready.get() />
                                <DashboardCardCircle
                                    label="Node CPU usage"
                                    label_add="used vs available"
                                    values=nodes_cpu.get()
                                    decimal=false />
                                <DashboardCardCircle
                                    label="Node Memory usage"
                                    label_add="used vs available"
                                    values=nodes_memory_values.get()
                                    value_labels=nodes_memory_labels.get()
                                    decimal=false />
                            </div>
                        </ExpandableSlot>
                    </Expandable>
                    <Expandable label="Pods" expanded=true>
                        <ExpandableSlot slot>
                            <div class="dashboard-card-container dcc-3">
                                <DashboardCardCircle
                                    label="Pods"
                                    label_add="ready vs requested"
                                    values=pods_ready.get() />
                                <DashboardCardCircle
                                    label="Pods CPU usage"
                                    label_add="actual vs reserved"
                                    values=pods_cpu.get()
                                    decimal=false />
                                <DashboardCardCircle
                                    label="Pods Memory usage"
                                    label_add="used vs available"
                                    values=pods_memory_values.get()
                                    value_labels=pods_memory_labels.get()
                                    decimal=false />
                            </div>
                        </ExpandableSlot>
                    </Expandable>
                    <Expandable label="Events" expanded=true>
                        <ExpandableSlot slot>
                            <div class="dashboard-card-container dcc-1">
                                <DashboardCardList
                                    labels=&["Type", "Name", "Time", "Reason", "Event"]
                                    widths=&["5%", "20%", "5%", "10%", "60%"]
                                    rows=events.get() />
                            </div>
                        </ExpandableSlot>
                    </Expandable>
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}

#[component]
pub fn DashboardCardCircle(
    label: &'static str,
    label_add: &'static str,
    values: (f64, f64),
    #[prop(default = (String::new(), String::new()))] value_labels: (String, String),
    #[prop(default = true)] decimal: bool,
) -> impl IntoView {
    let used_format = if decimal { format!("{:.0}", values.1) } else { format!("{:.2}", values.1) };
    let total_format = if decimal { format!("{:.0}", values.0) } else { format!("{:.2}", values.0) };

    view! {
        <div class="dashboard-card-circle">
            <div>
                <div class="label">{label}</div>
                <div class="label-add">{label_add}</div>
            </div>
            <div class="ring" style=format!("--fill: {}%", (values.1 / values.0) * 100.0)>
                <div class="ring-inner">
                    <div class="ring-inner-text">{used_format} {value_labels.1}</div>
                    <div class="ring-inner-text">of</div>
                    <div class="ring-inner-text">{total_format} {value_labels.0}</div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn DashboardCardList(
    labels: &'static[&'static str],
    widths: &'static[&'static str],
    rows: Vec<Vec<String>>,
) -> impl IntoView {
    view! {
        <div class="dashboard-card-list">
            <div class="table-header">
                <For
                    each=move || labels.iter().zip(widths).clone()
                    key=|(l, _)| l.to_string()
                    children=move |(l, w)| view! {
                        <div class="table-header-item" style=format!("width: {}", w.to_string())>{ l.to_string() }</div>
                    }
                />
            </div>
            <div class="table-body">
                <For
                    each=move || rows.clone()
                    key=|r| r.join(" ")
                    children=move |r| view! {
                        <div class="table-row">
                            <For
                                each=move || r.clone().into_iter().zip(widths).clone()
                                key=|(r, _)| r.clone()
                                children=move |(r, w)| view! {
                                    <div class="table-row-item" style=format!("width: {}", w.to_string())>{ r.to_string() }</div>
                                }
                            />
                        </div>
                    }
                />
            </div>
        </div>
    }
}
