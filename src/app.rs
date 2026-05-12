use gloo_console::log;
use yew::prelude::*;
use crate::components::digraph_tools::sidebar::Sidebar;
use crate::logic::digraph_logic::digest_values::{digest_values};
use crate::logic::digraph_logic::types::{DigestedValuesResult, GraphModes, ObjectSelection, ParseError};
use crate::render::digraph_render::graph::{Graph};

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
    // User determined state variables
    let input_value = use_state(String::new); // Used for controling user's input
    let graph_mode = use_state(|| GraphModes::DIGRAPH); // Tells us if we are rendering a digraph, matrix, or other
    let selection_object = use_state(|| ObjectSelection::default()); // Used for interactively selecting and highlighting elements

    // Calculated state variables
    let digested_values: UseStateHandle<DigestedValuesResult> = use_state(|| Err(ParseError::new("No input"))); // Parse user's input

    // Initialization actions
    let theme_css = generate_css_vars(); // Bind values from our theme json file to css variables to use in CSS

    // When a user changes their input value
    let on_input = {
        let input_value = input_value.clone();
        let digested_values = digested_values.clone();

        Callback::from(move |v: String| {
            // Set the input value
            input_value.set(v.clone());
            // Update the digested values for our pipeline
            digested_values.set(digest_values(v));
        })
    };

    // When a user toggles modes
    // Should probably live in <Graph/>, but we may use it in the future for rendring different info in the sidebar
    let on_mode_change: Callback<i32> = {
        let graph_mode = graph_mode.clone();
        Callback::from(move |int: i32| {
            graph_mode.set(match int {
                0 => {GraphModes::DIGRAPH},
                1 => {GraphModes::MATRIX},
                _ => {GraphModes::DIGRAPH}
            });
        })
    };

    html! {
        <div class="app">
            // Inject our variables
            <style>{ theme_css }</style>
            // Render the sidebar
            <Sidebar
                value={(*input_value).clone()}
                on_input={on_input}
                digested_values={digested_values.clone()}
                object_selection={selection_object.clone()}
            />
            // Render the graph
            <Graph
                digested_values={digested_values.clone()}
                mode_change_callback={on_mode_change}
                graph_mode={graph_mode.clone()}
                object_selection={selection_object}
            />
        </div>
    }
}