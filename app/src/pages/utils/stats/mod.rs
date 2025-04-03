pub mod pod_stats;

pub fn parse_memory(memory_str: &str) -> Option<f64> {
    let (num_str, unit_str) = memory_str.split_at(memory_str.len() - 2);
    let num: f64 = num_str.parse().ok()?;
    match unit_str {
        "Ki" => Some(num),
        "Mi" => Some(num * 1024.),
        "Gi" => Some(num * 1024. * 1024.),
        _ => None,
    }
}

pub fn convert_memory(bytes: f64) -> (f64, String) {
    let units = ["Ki", "Mi", "Gi", "Ti"];
    let mut unit_idx = 0;

    let mut memory_value = bytes;
    while memory_value >= 1024. && unit_idx < units.len() - 1 {
        memory_value /= 1024.;
        unit_idx += 1;
    }

    (memory_value, units[unit_idx].to_string())
}

pub fn parse_pod_cpu(request: &str) -> f64 {
    if request.ends_with("m") {
        let value: f64 = request.trim_end_matches("m").parse().unwrap_or(0.);
        value / 1000.
    } else {
        request.parse::<f64>().unwrap_or(0.)
    }
}
