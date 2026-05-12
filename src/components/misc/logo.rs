use yew::prelude::*;

use crate::components::misc::icon::Icon;

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    #[prop_or(false)]
    pub hide_logo: bool,
    #[prop_or(false)]
    pub hide_text: bool,
    #[prop_or(40)]
    pub size: i32,
    #[prop_or_default]
    pub class: String
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {

    html!{
        <div class={classes!("logo", props.class.clone())}>
            if !props.hide_logo {
                <Icon icon={"digraph"} size={props.size}/>
            }
            if !props.hide_text {
                <code style={format!("font-size: {}px;", props.size)}>{"Digrapher"}</code>
            }
        </div>
    }
}