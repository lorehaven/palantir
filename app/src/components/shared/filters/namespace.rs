use leptos::prelude::*;

use crate::components::prelude::*;

#[component]
pub fn NamespaceFilter(
    selected: RwSignal<String>,
    namespaces: RwSignal<Vec<String>>,
) -> impl IntoView {
    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="namespace-filter">
                    <select
                        prop:value=selected
                        on:change=move |ev| selected.set(event_target_value(&ev).parse().unwrap())
                    >
                        {namespaces.get().into_iter()
                            .map(|item| view! { <option> { item.to_string() } </option> })
                            .collect::<Vec<_>>()}
                    </select>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
