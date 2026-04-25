use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {

    let counter = use_state(||0);

    let onclick: Callback<MouseEvent> = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };
    
    html! {
        <div>
            {"Sidebar"}
            <button {onclick}>{"Click me"}</button>
            <p>{*counter}</p>
        </div>
    }
}