use api::cluster::namespaces as namespaces_api;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::prelude::*;

#[component]
pub fn NamespacesFilterAction(selected_namespace: RwSignal<String>) -> impl IntoView {
    let namespaces = RwSignal::new(vec![]);

    Effect::new(move |_| {
        spawn_local(async move {
            let mut namespaces_names = namespaces_api::get_namespaces()
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|n| n.metadata.name)
                .collect::<Vec<_>>();
            namespaces_names.insert(0, "All Namespaces".to_string());
            namespaces.set(namespaces_names);
        });
    });

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action namespaces-action">
                    <select
                        prop:value=selected_namespace
                        on:change=move |ev| selected_namespace.set(event_target_value(&ev).parse().unwrap())
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
