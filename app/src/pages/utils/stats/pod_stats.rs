use crate::domain::metrics::PodMetrics;
use crate::domain::cluster::pod::Pod;
use crate::pages::utils::stats::{convert_memory, parse_memory, parse_pod_cpu};

pub fn pod_cpu_actual(metrics: &PodMetrics) -> String {
    let usage = parse_pod_cpu_actual_f64(metrics);
    if usage == 0. { "0m".to_string() }
    else { format!("{:.2}m", parse_pod_cpu_actual_f64(metrics) / 1_000_000.) }
}

pub fn pod_cpu_request(pod: &Pod, metrics: &PodMetrics) -> String {
    let request = pod.spec.containers.iter()
        .fold(0., |acc, c| acc + parse_pod_cpu(&c.resources.requests.cpu));
    let usage = parse_pod_cpu_actual_f64(metrics);
    let usage_percentage =
        if request == 0. { "-".to_string() }
        else { format!("{:.2}%", usage / 10_000_000. / request) };
    format!("{usage_percentage}\n{}m", request * 1000.)
}

pub fn pod_cpu_limit(pod: &Pod, metrics: &PodMetrics) -> String {
    let request = pod.spec.containers.iter()
        .fold(0., |acc, c| acc + parse_pod_cpu(&c.resources.limits.cpu));
    let usage = parse_pod_cpu_actual_f64(metrics);
    if request == 0. { "-".to_string() }
    else { format!("{:.2}%\n{}m", usage / 10_000_000. / request, request * 1000.) }
}

fn parse_pod_cpu_actual_f64(metrics: &PodMetrics) -> f64 {
    metrics
        .containers.iter()
        .fold(0., |acc, c| acc + c.usage.cpu.trim_end_matches('n').parse::<f64>().unwrap_or(0.))
}

pub fn pod_memory_actual(metrics: &PodMetrics) -> String {
    let (value, suffix) = convert_memory(parse_pod_memory_actual_f64(metrics));
    if value == 0. { "0Bi".to_string() }
    else { format!("{value:.2}{suffix}") }
}

pub fn pod_memory_request(pod: &Pod, metrics: &PodMetrics) -> String {
    let request = pod.spec.containers.iter()
        .filter(|c| !c.resources.requests.memory.is_empty())
        .fold(0., |acc, c| acc + parse_memory(&c.resources.requests.memory).unwrap_or_default());
    let actual = parse_pod_memory_actual_f64(metrics);
    let request_percentage = actual / request * 100.;
    let (request, suffix) = convert_memory(request);
    if request == 0. { "-".to_string() }
    else { format!("{request_percentage:.2}%\n{request}{suffix}") }
}

pub fn pod_memory_limit(pod: &Pod, metrics: &PodMetrics) -> String {
    let limit = pod.spec.containers.iter()
        .filter(|c| !c.resources.limits.memory.is_empty())
        .fold(0., |acc, c| acc + parse_memory(&c.resources.limits.memory).unwrap_or_default());
    let actual = parse_pod_memory_actual_f64(metrics);
    let limit_percentage = actual / limit * 100.;
    let (limit, suffix) = convert_memory(limit);
    if limit == 0. { "-".to_string() }
    else { format!("{limit_percentage:.2}%\n{limit}{suffix}") }
}

fn parse_pod_memory_actual_f64(metrics: &PodMetrics) -> f64 {
    metrics
        .containers.iter()
        .fold(0., |acc, c| acc + parse_memory(&c.usage.memory).unwrap_or_default())
}
