use yew::prelude::*;

use crate::components::misc::icon::Icon;

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    #[prop_or(false)]
    pub hide_logo: bool,
    #[prop_or(40)]
    pub size: i32
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {

    html!{
        <div class={"logo"}>
            <Icon icon={"digraph"} size={props.size}/>
            <code>{"Digrapher"}</code>
        </div>
    }
}