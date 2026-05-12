use yew::prelude::*;

use crate::components::navigation::topbar_layout::TopbarLayout;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <TopbarLayout>
            <div class="not-found">
                <h1 class="not-found__heading">{ "404 Not found" }</h1>
                <code class="not-found__description">{ "The page you were looking for could not be located." }</code>
            </div>
        </TopbarLayout>
    }
}