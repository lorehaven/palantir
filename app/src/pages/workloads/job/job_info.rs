use api::workloads::jobs as jobs_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;
use crate::utils::shared::display;
use crate::utils::shared::effects::{clear_page_effect, update_page_effect};
use crate::utils::shared::time::format_timestamp;

#[component]
pub fn JobInfoComponent(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let data = RwSignal::new(vec![]);

    let interval_handle = update_page_effect(10_000, move || {
        update_page(namespace_name, resource_name, data);
    });
    clear_page_effect(interval_handle);

    resource_info_view(data)
}

fn update_page(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    data: RwSignal<Vec<(String, String)>>,
) {
    if namespace_name.is_disposed() || resource_name.is_disposed() {
        return;
    }
    let namespace_name = namespace_name.get();
    let resource_name = resource_name.get();

    spawn_local(async move {
        let job = jobs_api::get_jobs(None)
            .await
            .unwrap_or_default();
        let job = job
            .into_iter()
            .find(|n| n.metadata.namespace == namespace_name && n.metadata.name == resource_name)
            .unwrap_or_default();

        let start_time = job.clone().status.start_time;
        let end_time = job.clone().status.completion_time;

        data.set(
            vec![
                ("Name", job.clone().metadata.name),
                ("Kind", "Job".to_string()),
                ("Namespace", job.clone().metadata.namespace),
                (
                    "Created",
                    format_timestamp(
                        &job
                            .clone()
                            .metadata
                            .creation_timestamp
                            .unwrap_or_default(),
                        None,
                    ),
                ),
                (
                    "Labels",
                    display::hashmap(job.clone().metadata.labels),
                ),
                (
                    "Annotations",
                    display::hashmap(job.clone().metadata.annotations),
                ),
                ("Version", job.metadata.resource_version),
                ("Start Time", start_time.clone()),
                ("Completion Time", end_time.clone()),
                ("Duration", job_duration(&start_time, &end_time)),
            ]
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        );
    });
}

fn job_duration(start_time: &str, end_time: &str) -> String {
    let start: chrono::DateTime<chrono::Utc> =
        if let Ok(time) = start_time.parse() { time }
        else { return "-".to_string(); };
    let end: chrono::DateTime<chrono::Utc> =
        if let Ok(time) = end_time.parse() { time }
        else { return "-".to_string(); };

    let duration = end - start;

    let total_millis = duration.num_milliseconds();
    let hours = total_millis / 3_600_000;
    let minutes = (total_millis % 3_600_000) / 60_000;
    let seconds = (total_millis % 60_000) / 1_000;
    let millis = total_millis % 1_000;

    let mut parts = Vec::new();
    if hours != 0 {
        parts.push(format!("{hours}h"));
    }
    if minutes != 0 {
        parts.push(format!("{minutes}m"));
    }
    if seconds != 0 {
        parts.push(format!("{seconds}s"));
    }
    if millis != 0 {
        parts.push(format!("{millis}ms"));
    }

    if parts.is_empty() {
        "0ms".to_string()
    } else {
        parts.join(" ")
    }
}
