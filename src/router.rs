use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{
    home::HomePage,
    digraph::DigraphPage,
    not_found::NotFoundPage
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/digraph")]
    Digraph,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Digraph => html! { <DigraphPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}