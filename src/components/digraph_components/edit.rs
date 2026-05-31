use yew::prelude::*;

use crate::{components::misc::tooltip::Tooltip, services::digraph_services::types::GraphTooltips};

#[derive(PartialEq, Properties)]
pub struct RelationEditProps {
    pub graph_editing_mode: UseStateHandle<Option<GraphTooltips>>,

}

#[function_component(RelationEdit)]
pub fn relation_edit(props: &RelationEditProps) -> Html {
    let selected_tooltip = props.graph_editing_mode.clone();

    let tooltip_types: Vec<(&str, &str)> = vec![("Move", "move"), ("New point", "point"), ("Connect edge", "edge"), ("Edit title", "pencil"), ("Delete point", "backspace")];

    let onclick: Callback<i32> = {
        let selected = selected_tooltip.clone();
        Callback::from(move |id: i32| {
            selected.set(Some(GraphTooltips::from_i32(id)));
        })
    };


    html! {
        <div class="edit">
            <code class="edit__description">
                {match tooltip_types.get(selected_tooltip.unwrap_or(GraphTooltips::MOVE).to_i32() as usize){
                    Some((description, _)) => {
                        description.to_string()
                    },
                    _ => {
                        String::from("No description")
                    }
                }}
            </code>
            <div class="edit__tooltips">
                {for tooltip_types.iter().enumerate().map(|(index, (name, icon))| {
                    html! {
                        <Tooltip
                            id={index as i32}
                            onclick={onclick.clone()}
                            title={*name}
                            selected={selected_tooltip.unwrap_or(GraphTooltips::MOVE).to_i32() == index as i32}
                            icon={icon.to_string()}
                            size={24}
                        />
                    }
                })}
            </div>
        </div>
    }
}