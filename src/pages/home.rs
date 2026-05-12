use yew::prelude::*;

use crate::components::navigation::topbar_layout::TopbarLayout;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <TopbarLayout>
            <h1>{ "Home" }</h1>
        </TopbarLayout>
    }
}