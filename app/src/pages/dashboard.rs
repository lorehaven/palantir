use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::pages::components::dashboard::prelude::*;
use crate::pages::components::prelude::*;
use crate::pages::utils::dashboard::*;
use crate::pages::utils::shared::effects::update_page_effect;
use crate::pages::utils::shared::time::time_until_now;

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

    update_page_effect(10_000, move || update_page(
        nodes_ready.clone(), nodes_cpu.clone(), nodes_memory_values.clone(), nodes_memory_labels.clone(),
        pods_ready.clone(), pods_cpu.clone(), pods_memory_values.clone(), pods_memory_labels.clone(),
        events.clone(), loading.clone()));

    view(
        nodes_ready,
        nodes_cpu,
        nodes_memory_values,
        nodes_memory_labels,
        pods_ready,
        pods_cpu,
        pods_memory_values,
        pods_memory_labels,
        events,
    )
}

fn update_page(
    nodes_ready: RwSignal<(f64, f64)>,
    nodes_cpu: RwSignal<(f64, f64)>,
    nodes_memory_values: RwSignal<(f64, f64)>,
    nodes_memory_labels: RwSignal<(String, String)>,
    pods_ready: RwSignal<(f64, f64)>,
    pods_cpu: RwSignal<(f64, f64)>,
    pods_memory_values: RwSignal<(f64, f64)>,
    pods_memory_labels: RwSignal<(String, String)>,
    events: RwSignal<Vec<Vec<String>>>,
    loading: RwSignal<bool>,
) {
    spawn_local(async move {
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
    });
}

fn view(
    nodes_ready: RwSignal<(f64, f64)>,
    nodes_cpu: RwSignal<(f64, f64)>,
    nodes_memory_values: RwSignal<(f64, f64)>,
    nodes_memory_labels: RwSignal<(String, String)>,
    pods_ready: RwSignal<(f64, f64)>,
    pods_cpu: RwSignal<(f64, f64)>,
    pods_memory_values: RwSignal<(f64, f64)>,
    pods_memory_labels: RwSignal<(String, String)>,
    events: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
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
