use std::ops::Deref;

use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::router::{switch, Route};
use crate::components::contexts::theme_context::{ThemeContext, StyleProvider};

#[function_component(App)]
pub fn app() -> Html {
    // Get localstorage theme that was set
    let stored_theme = LocalStorage::get::<String>("theme").unwrap_or("light".to_string());
    let theme = use_state(||stored_theme);

    let context = ThemeContext {
        theme: theme.deref().clone(),
        set_theme: {
            let theme = theme.clone();
            Callback::from(move |new_theme: String| {
                LocalStorage::set("theme", &new_theme).unwrap();
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