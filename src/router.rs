use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{
    home::HomePage,
    not_found::NotFoundPage,
    digraph::DigraphPage,
    matrix::MatrixPage
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,

    #[at("/digraph")]
    Digraph,

    #[at("/matrix")]
    Matrix
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::NotFound => html! { <NotFoundPage /> },

        Route::Digraph => html! { <DigraphPage /> },
        Route::Matrix => html! { <MatrixPage /> }
    }
}