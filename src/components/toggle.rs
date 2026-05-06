use gloo_console::log;
use yew::prelude::*;

use crate::components::icon::{Icon};

#[derive(Properties, PartialEq)]
pub struct ToggleProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(40)]
    pub size: i32,
    pub onchange: Callback<i32>
}

#[function_component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    let selected = use_state(|| 0);
    let selected_clone = selected.clone();

    html!{
    <div class={classes!("toggle", props.class.clone())}>
        {for props.children.iter().enumerate().map(|(i, child)| {
            let selected = selected_clone.clone();

            let onclick = {
                let selected = selected.clone();
                let onchange = props.onchange.clone();
                Callback::from(move |_| {
                    selected.set(i as i32);
                    onchange.emit(i as i32);
                })
            };

            html! {
                <button
                    class={classes!("toggle__option", props.class.clone())}
                    {onclick}
                >
                    {child}
                </button>
            }
        })}
            <div
                class="toggle__selected"
                style={format!("width: {}px; height: {}px; left: {}%", props.size, props.size, (*selected).clone() * 100 / props.children.len() as i32)}
            ></div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ToggleOptionProps {
    #[prop_or_default]
    pub class: Classes,
    pub icon: String,
    #[prop_or(40)]
    pub size: i32,
}

#[function_component(ToggleOption)]
pub fn toggle_option(props: &ToggleOptionProps) -> Html {
    html!{
        <Icon icon={props.icon.clone()} size={props.size} color="onSurface"/>
    }
}