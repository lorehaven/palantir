use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn NamespaceFilter(
    namespace_name: RwSignal<String>,
    namespaces: RwSignal<Vec<String>>,
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="namespace-filter">
                    <select
                        prop:value=namespace_name
                        on:change=move |ev| namespace_name.set(event_target_value(&ev).parse().unwrap())
                    >
                        {namespaces.get().into_iter()
                            .map(|item| view! { <option> { item } </option> })
                            .collect::<Vec<_>>()}
                    </select>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
