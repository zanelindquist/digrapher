use yew::prelude::*;

use crate::components::navigation::{
    topbar_layout::TopbarLayout,
    footer::Footer
};

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {

    // Set page name
    use_effect(|| {
        gloo_utils::document().set_title("404 Not Found");
        || ()
    });

    html! {
        <TopbarLayout class="not-found__container">
            <div class="not-found dynamic-padding">
                <h1 class="not-found__heading">{ "404 Not found" }</h1>
                <code class="not-found__description">{ "The page you were looking for could not be located." }</code>
                <img class="not-found__image" src={format!("/assets/misc/404_panda.png")}/>
            </div>
            <Footer/>
        </TopbarLayout>
    }
}