use api::resource as resource_api;
use leptos::prelude::*;
use leptos::task::spawn_local;
use domain::cluster::pod::Container;
use crate::components::prelude::*;

#[component]
pub fn ContainersFilterAction(
    resource_type: RwSignal<String>,
    namespace_name: RwSignal<String>,
    resource_name: RwSignal<String>,
    selected_container: RwSignal<String>,
) -> impl IntoView {
    let namespace_name = or_none(namespace_name);
    let resource_name = or_none(resource_name);
    let containers = RwSignal::new(vec![]);

    Effect::new(move |_| {
        spawn_local(async move {
            let res = resource_api::get(
                resource_type.get_untracked(),
                namespace_name.get_untracked(),
                resource_name.get_untracked(),
            )
                .await
                .unwrap_or_default();
            let json_value = serde_json::from_str::<serde_json::Value>(&res).unwrap();
            let data = json_value["spec"]["containers"].as_array().unwrap()
                .iter()
                .map(|c| serde_json::from_value::<Container>(c.clone()).unwrap_or_default().name)
                .collect::<Vec<String>>();
            selected_container.set(data.first().cloned().unwrap_or_default());
            containers.set(data);
        });
    });

    view! {
        <Wrapper>
            <WrapperSlot slot>
                <div class="action containers-action">
                    <select
                        prop:value=namespace_name
                        on:change=move |ev| selected_container.set(event_target_value(&ev).parse().unwrap())
                    >
                        {containers.get().into_iter()
                            .map(|item| view! { <option> { item } </option> })
                            .collect::<Vec<_>>()}
                    </select>
                </div>
            </WrapperSlot>
        </Wrapper>
    }
}
