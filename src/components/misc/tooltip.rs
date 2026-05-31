use yew::prelude::*;

use crate::components::misc::icon::Icon;


#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub id: i32,
    pub onclick: Callback<i32>,
    pub selected: bool,

    #[prop_or(String::from("No description"))]
    pub title: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or(String::from("question-mark"))]
    pub icon: String,
    #[prop_or(40)]
    pub size: i32,
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    html!{
        <button
            class={classes!(
                "button",
                "tooltip", 
                if props.selected {"tooltip--selected"} else {""},
                props.class.clone()
            )}
            onclick={
                let onclick = props.onclick.clone();
                let id = props.id.clone();
                Callback::from(move |_| {
                    onclick.emit(id);
                })
            }
            title={props.title.clone()}
        >
            <Icon color="onPrimaryContainer" icon={props.icon.clone()} size={props.size}/>
        </button>
    }
}