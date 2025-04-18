use api::workloads as workloads_api;
use api::workloads::pods as pods_api;
use domain::utils::time::time_until_now;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};

pub mod configmap;
pub mod configmaps;
pub mod deployment;
pub mod ingress;
pub mod ingresses;
pub mod job;
pub mod pod;
pub mod pod_logs;
pub mod pods;
pub mod replica;
pub mod replicas;
pub mod service;
pub mod services;

#[component]
pub fn WorkloadsPage() -> impl IntoView {
    let resource_type = RwSignal::new("Workloads".to_string());
    let resource_name = RwSignal::new(String::new());
    let namespace_name = RwSignal::new("All Namespaces".to_string());

    view! {
        <Header text=vec!["Workloads"] />
        <PageContent>
            <PageContentSlot slot>
                <div class="workloads main-page">
                    <Actions
                        resource_type
                        selected_namespace=namespace_name
                        prompt_value=resource_name
                        actions=&[ActionType::NamespacesFilter, ActionType::Prompt] />
                    <WorkloadsStats namespace_name />
                    <WorkloadsList namespace_name resource_name />
                </div>
            </PageContentSlot>
        </PageContent>
        <Footer />
    }
}

#[component]
fn WorkloadsStats(namespace_name: RwSignal<String>) -> impl IntoView {
    let workloads_ready = RwSignal::new((0., 0.));
    let pods_ready = RwSignal::new((0., 0.));

    let interval_handle = update_page_effect(10_000, move || {
        update_page_stats(namespace_name, workloads_ready, pods_ready);
    });
    clear_page_effect(interval_handle);

    view_stats(workloads_ready, pods_ready)
}

fn update_page_stats(
    namespace_name: RwSignal<String>,
    workloads_ready: RwSignal<(f64, f64)>,
    pods_ready: RwSignal<(f64, f64)>,
) {
    if namespace_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();

    spawn_local(async move {
        let namespace_name = if namespace_name == "All Namespaces" {
            None
        } else {
            Some(namespace_name)
        };
        let workloads = workloads_api::get_workloads(namespace_name.clone()).await;
        let ready_workloads = workloads.iter().filter(|w| w.is_ready()).count();
        workloads_ready.set((workloads.len() as f64, ready_workloads as f64));

        let pods = pods_api::get_pods(namespace_name, None)
            .await
            .unwrap_or_default();
        let ready_pods = pods
            .iter()
            .filter(|p| {
                p.status
                    .conditions
                    .iter()
                    .any(|pc| pc.r#type == "Ready" && pc.status == "True")
            })
            .count();
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
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let table_rows = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let columns = vec![
        TableColumn::new("Type", TableColumnType::String, 1),
        TableColumn::new("Namespace", TableColumnType::Link, 1),
        TableColumn::new("Name", TableColumnType::Link, 2),
        TableColumn::new("Age", TableColumnType::String, 1),
        TableColumn::new("Pods", TableColumnType::String, 3),
    ];
    let styles = vec![String::new(); columns.len()];
    let mut params = vec![String::new(); columns.len()];
    params[1] = "/cluster/namespaces/".to_string();
    params[2] = "/workloads/:1/:0s/".to_string();

    let columns_update = columns.clone();
    let interval_handle = update_page_effect(10_000, move || {
        update_page_list(
            columns_update.clone(),
            styles.clone(),
            params.clone(),
            table_rows,
            namespace_name,
            resource_name,
            loading,
        );
    });
    clear_page_effect(interval_handle);
    data_list_view(columns, table_rows, loading)
}

fn update_page_list(
    columns: Vec<TableColumn>,
    styles: Vec<String>,
    params: Vec<String>,
    table_rows: RwSignal<Vec<TableRow>>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    loading: RwSignal<bool>,
) {
    if namespace_name.is_disposed() || resource_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let resource_name = resource_name.get();

    spawn_local(async move {
        let list = update_page_list_async(
            columns.clone(),
            styles.clone(),
            params.clone(),
            namespace_name.clone(),
            resource_name.clone(),
        )
        .await
        .unwrap_or_default();
        table_rows.set(list);
        loading.set(false);
    });
}

#[server]
async fn update_page_list_async(
    columns: Vec<TableColumn>,
    styles: Vec<String>,
    params: Vec<String>,
    namespace_name: String,
    resource_name: String,
) -> Result<Vec<TableRow>, ServerFnError> {
    let namespace_name = if namespace_name == "All Namespaces" {
        None
    } else {
        Some(namespace_name)
    };
    let mut list = workloads_api::get_workloads(namespace_name)
        .await
        .into_iter()
        .filter(|w| {
            w.get_name()
                .to_lowercase()
                .contains(&resource_name.to_lowercase())
        })
        .map(|w| w.to_model())
        .map(|w| vec![w.r#type, w.namespace, w.name, w.age, w.pods])
        .collect::<Vec<_>>();

    list.sort_by(|a, b| a[1].cmp(&b[1]));
    Ok(parse_table_rows(columns, list, styles, params))
}
