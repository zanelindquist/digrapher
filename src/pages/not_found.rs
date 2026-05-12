use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div>
            <h1>{ "404 Not found" }</h1>
        </div>
    }
}