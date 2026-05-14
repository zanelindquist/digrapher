use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    pub onclick: Callback<MouseEvent>
}


#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {

    html! {
        <button
            onclick={props.onclick.clone()}
            class={classes!("button", props.class.clone())}
        >
            {props.children.clone()}
        </button>
    }
}