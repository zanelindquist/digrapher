use yew::prelude::*;
use crate::components::sidebar::Sidebar;
use crate::logic::digest_values::{digest_values};
use crate::logic::types::{DigestedValuesResult, ParseError};
use crate::render::graph::{Graph};

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
    let theme_css = generate_css_vars();
    let input_value = use_state(String::new);

    let digested_values: UseStateHandle<DigestedValuesResult> = use_state(|| Err(ParseError::new("No input")));

    let on_input = {
        let input_value = input_value.clone();
        let digested_values = digested_values.clone();

        Callback::from(move |v: String| {
            input_value.set(v.clone());
            digested_values.set(digest_values(v));
        })
    };

    html! {
        <div class="app">
            <style>{ theme_css }</style>

            <Sidebar
                value={(*input_value).clone()}
                on_input={on_input}
                digested_values={digested_values.clone()}
            />

            <Graph
                digested_values={digested_values.clone()}
            />
        </div>
    }
}