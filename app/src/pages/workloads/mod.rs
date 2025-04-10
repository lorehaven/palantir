use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::workloads::pods as pods_api;
use crate::api::workloads as workloads_api;
use crate::components::prelude::*;
use crate::pages::utils::shared::effects::{clear_page_effect, update_page_effect};

pub mod configmap;
pub mod configmaps;
pub mod ingress;
pub mod ingresses;
pub mod pod;
pub mod pods;
pub mod replica;
pub mod replicas;
pub mod service;
pub mod services;

#[component]
pub fn WorkloadsPage() -> impl IntoView {
    let prompt = RwSignal::new(String::new());
    let selected = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads main-page">
                    <Filter
                        label="Workloads"
                        selected
                        prompt
                        with_namespace=true
                        with_prompt=true />
                    <WorkloadsStats selected />
                    <WorkloadsList selected prompt />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}

#[component]
fn WorkloadsStats(
    selected: RwSignal<String>,
) -> impl IntoView {
    let workloads_ready = RwSignal::new((0., 0.));
    let pods_ready = RwSignal::new((0., 0.));

    let interval_handle = update_page_effect(3_600_000, move || update_page_stats(
        selected,
        workloads_ready,
        pods_ready,
    ));
    clear_page_effect(interval_handle);

    view_stats(workloads_ready, pods_ready)
}

fn update_page_stats(
    selected: RwSignal<String>,
    workloads_ready: RwSignal<(f64, f64)>,
    pods_ready: RwSignal<(f64, f64)>,
) {
    if selected.is_disposed() { return; }
    let selected_value = selected.get();

    spawn_local(async move {
        let workloads =
            if selected_value == "All Namespaces" { workloads_api::get_workloads(None).await }
            else { workloads_api::get_workloads(Some(selected_value.clone())).await };
        let ready_workloads = workloads.iter()
            .filter(|w| w.is_ready()).count();
        workloads_ready.set((workloads.len() as f64, ready_workloads as f64));

        let pods =
            if selected_value == "All Namespaces" { pods_api::get_pods().await.unwrap_or_default() }
            else { pods_api::get_pods_by_namespace_name(selected_value).await.unwrap_or_default() };
        let ready_pods = pods.iter()
            .filter(|p| p.status.conditions.iter().any(|pc| pc.r#type == "Ready" && pc.status == "True")).count();
        pods_ready.set((pods.len() as f64, ready_pods as f64));
    });
}

fn view_stats(
    workloads_ready: RwSignal<(f64, f64)>,
    pods_ready: RwSignal<(f64, f64)>,
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-2">
                    <CardCircle
                        label="Workloads"
                        label_add="ready vs requested"
                        values=workloads_ready.get() />
                    <CardCircle
                        label="Pods"
                        label_add="ready vs requested"
                        values=pods_ready.get() />
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}

#[component]
fn WorkloadsList(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
) -> impl IntoView {
    let workloads = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(3_600_000, move || update_page_list(
        selected,
        prompt,
        workloads,
    ));
    clear_page_effect(interval_handle);

    view_list(workloads)
}

fn update_page_list(
    selected: RwSignal<String>,
    prompt: RwSignal<String>,
    workloads: RwSignal<Vec<Vec<String>>>,
) {
    let selected_value = selected.get();
    let prompt_value = prompt.get();
    spawn_local(async move {
        let workloads_list =
            if selected_value == "All Namespaces" { workloads_api::get_workloads(None).await }
            else { workloads_api::get_workloads(Some(selected_value.clone())).await };
        let mut list = workloads_list.into_iter()
            .filter(|w| w.get_name().to_lowercase().contains(&prompt_value.to_lowercase()))
            .map(|w| w.to_model())
            .map(|w| vec![
            w.r#type, w.name, w.namespace, w.age, w.pods,
        ]).collect::<Vec<_>>();
        list.sort_by(|a, b| a[1].cmp(&b[1]));
        workloads.set(list);
    });
}

fn view_list(
    workloads: RwSignal<Vec<Vec<String>>>,
) -> impl IntoView {
    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Name", TableColumnType::Link, 2),
        TableColumn::new("Namespace", TableColumnType::Link, 1),
        TableColumn::new("Age", TableColumnType::String, 1),
        TableColumn::new("Pods", TableColumnType::String, 3),
    ];
    let styles = vec![""; columns.len()];
    let mut params = vec![""; columns.len()];
    params[1] = "/workloads/:0/:2/";
    params[2] = "/cluster/namespaces/";

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="card-container dcc-1">
                    <div class="card-table">
                        <TableComponent
                            columns=columns.clone()
                            values=workloads.get()
                            styles=styles.clone()
                            params=params.clone() />
                    </div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
