use leptos::prelude::*;

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
        <div class="card-circle">
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
