use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn ExecAction(
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
) -> impl IntoView {
    let url = format!(
        "/workloads/{}/pods/{}/exec",
        namespace_name.get_untracked(),
        resource_name.get_untracked()
    );

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action exec-action">
                    <a href=url.clone() class="actions-icon">
                        <i class="fa-solid fa-terminal" />
                    </a>
                    <div>Exec</div>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
