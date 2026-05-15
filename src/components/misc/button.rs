use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub disabled: bool
}


#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {

    html! {
        <button
            onclick={props.onclick.clone()}
            class={classes!("button", props.class.clone())}
            disabled={props.disabled.clone()}
        >
            {props.children.clone()}
        </button>
    }
}