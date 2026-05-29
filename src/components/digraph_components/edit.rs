use yew::prelude::*;

use crate::{components::misc::tooltip::Tooltip, services::digraph_services::types::GraphTooltips};

#[derive(PartialEq, Properties)]
pub struct RelationEditProps {

}

#[function_component(RelationEdit)]
pub fn relation_edit(props: &RelationEditProps) -> Html {
    let selected_tooltip = use_state(|| GraphTooltips::NEW_POINT);

    let tooltip_types: Vec<(&str, &str)> = vec![("New point", "point"), ("Connect edge", "edge"), ("Edit Title", "pencil"), ("Delete element", "backspace")];

    let onclick: Callback<i32> = {
        let selected = selected_tooltip.clone();
        Callback::from(move |id: i32| {
            selected.set(GraphTooltips::from_i32(id));
        })
    };

    html! {
        <div class="edit">
            <div class="edit__description">
                <p>
                </p>
            </div>
            <div class="edit__tooltips">
                {for tooltip_types.iter().enumerate().map(|(index, (name, icon))| {
                    html! {
                        <Tooltip
                            id={index as i32}
                            onclick={onclick.clone()}
                            title={*name}
                            selected={(*selected_tooltip).to_i32() == index as i32}
                            icon={icon.to_string()}
                            size={24}
                        />
                    }
                })}
            </div>
        </div>
    }
}