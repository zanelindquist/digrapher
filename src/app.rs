use yew::prelude::*;
use crate::components::sidebar::Sidebar;
use crate::render::graph::{Graph, GraphProps};

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
    
    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |v: String| {
            input_value.set(v);
            web_sys::console::log_1(&format!("UPDATE").into());
        })
    };

    html! {
        <div class="app">
            <style>{ theme_css }</style>
            <Sidebar value={(*input_value).clone()} on_input={on_input.clone()}/>
            <Graph input={(*input_value).clone()}/>
        </div>
    }
}