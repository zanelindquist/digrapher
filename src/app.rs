use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};

// Theme embedded at compile time
const THEME_JSON: &str = include_str!("assets/themes/organic.json");

fn generate_css_vars() -> String {
    let value: serde_json::Value = serde_json::from_str(THEME_JSON).unwrap();
    
    if let Some(schemes) = value.get("schemes") {
        if let Some(light) = schemes.get("light") {
            if let Some(obj) = light.as_object() {
                let mut css = String::from(":root {\n");
                for (key, val) in obj {
                    if let Some(s) = val.as_str() {
                        css.push_str(&format!("  --{}: {};\n", key, s));
                    }
                }
                css.push_str("}\n");
                return css;
            }
        }
    }
    String::new()
}

#[function_component(App)]
pub fn app() -> Html {
    // Initialization actions
    let theme_css = generate_css_vars(); // Bind values from our theme json file to css variables to use in CSS

    html! {
        <div class="app">
            // Inject our variables
            <style>{ theme_css }</style>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
            // <HomePage/>
        </div>
    }
}