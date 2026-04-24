use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello Yew" }</h1>
            <button>{"Click me"}</button>        
        </>
    }
}