use web_sys::{MouseEvent};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::misc::logo::Logo;
use crate::components::contexts::theme_context::StyleToggle;
use crate::router::Route;

const RELEASES_JSON: &str = include_str!("../../assets/supported_releases.json");

#[derive(serde::Deserialize)]
struct Release {
    pub version: &'static str,
    pub url: &'static str,
    pub date: &'static str
}

#[derive(Properties, PartialEq)]
pub struct TopbarProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes
}

#[function_component(TopbarLayout)]
pub fn topbar_layout(props: &TopbarProps) -> Html {
    let releases: Vec<Release> = serde_json::from_str(RELEASES_JSON).unwrap_or_else(|_| vec![Release {version: "None", url: "not_found", date: "0-0-0000"}]);

    let versions_open = use_state(|| false);

    let version_onclick = {
        let versions_open = versions_open.clone();
        Callback::from(move |event: MouseEvent| {
            versions_open.set(!*versions_open);
            event.prevent_default();
        })
    };

    let on_select_version = {
        let versions_open = versions_open.clone();
        Callback::from(move |route: &'static str| {
            // gloo_utils::window().location().set_href(&format!("/{}", route).to_string());
            versions_open.set(false);
        })
    };

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
                    <StyleToggle/>
                    {if *versions_open {
                        html! {
                            <div class="topbar__status__dropdown">
                                {for releases.iter().enumerate().map(|(index, release)| {
                                    let is_latest = index == 0;
                                    let is_current = release.version == env!("CARGO_PKG_VERSION");

                                    let on_select_version = on_select_version.clone();
                                    let url = if is_latest {"/"} else if is_current {""} else {release.url};
                                    html! {
                                        <button
                                            class={classes!("topbar__status__dropdown__item", if is_current {"topbar__status__dropdown__item--selected"} else {""})}
                                            style={
                                                if index == 0 {
                                                    "border-top-left-radius: 10px; border-top-right-radius: 10px;"
                                                } else if index == releases.len() - 1 {
                                                    " border-bottom-left-radius: 10px; border-bottom-right-radius: 10px;"
                                                } else {
                                                    ""
                                                }
                                            }
                                            onclick={move |_| {
                                                on_select_version.emit(url)
                                            }}
                                        >
                                            {format!("v{}{}", release.version, if is_latest {" (latest)"} else {""})}
                                        </button>
                                    }
                                })}
                            </div>
                        }
                    }else {html!{}}}
                    <button
                        class="topbar__status"
                        onclick={version_onclick}
                    >
                        <span class="topbar__dot"></span>
                        <code>{format!("v{}", env!("CARGO_PKG_VERSION"))}</code>
                    </button>
                </div>
            </header>
            <main class={classes!("topbar__content", props.class.clone())}>
                {props.children.clone()}
            </main>
        </div>
    }
}