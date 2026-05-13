use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};
use crate::components::contexts::theme_context::{ThemeContext, StyleProvider};

#[function_component(App)]
pub fn app() -> Html {
    let theme = use_state(||"light".to_string());

    let context = ThemeContext {
        theme: (*theme).clone(),
        set_theme: {
            let theme = theme.clone();

            Callback::from(move |new_theme: String| {
                theme.set(new_theme)
            })
        },
    };

    html! {
        <div class="app">
            // Inject our variables
            <ContextProvider<ThemeContext> context={context}>
                <StyleProvider/>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<ThemeContext>>
        </div>
    }
}