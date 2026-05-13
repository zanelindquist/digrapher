use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::misc::logo::Logo;
use crate::components::navigation::footer::Footer;
use crate::router::Route;


#[derive(Properties, PartialEq)]
pub struct TopbarProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes
}

#[function_component(TopbarLayout)]
pub fn topbar_layout(props: &TopbarProps) -> Html {
    html! {
        <div class="topbar__layout">
            <header class="topbar">
                <div class="topbar__left">
                    <Logo size={22} class="topbar__logo"/>
                </div>
                <nav class="topbar__nav">
                    <Link<Route>
                        to={Route::Home}
                        classes="topbar__link"
                    >
                        {"home"}
                    </Link<Route>>
                    <Link<Route>
                        to={Route::Digraph}
                        classes="topbar__link"
                    >
                        {"digraph"}
                    </Link<Route>>
                </nav>
                <div class="topbar__right">
                    <div class="topbar__status">
                        <span class="topbar__dot"></span>
                        <code>{"stable"}</code>
                    </div>
                </div>
            </header>
            <main class={classes!("topbar__content", props.class.clone())}>
                {props.children.clone()}
            </main>
        </div>
    }
}