use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn LogsAction(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let url = format!(
        "/workloads/{}/pods/{}/logs",
        namespace_name.get_untracked(),
        resource_name.get_untracked()
    );

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action logs-action">
                    <a href=url.clone() class="actions-icon">
                        <i class="fa-solid fa-cloud-arrow-down" />
                    </a>
                    <div>Logs</div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
