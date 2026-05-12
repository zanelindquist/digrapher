use gloo_console::log;
use yew::prelude::*;
use crate::components::digraph_tools::sidebar::Sidebar;
use crate::components::navigation::topbar_layout::TopbarLayout;
use crate::logic::digraph_logic::digest_values::{digest_values};
use crate::logic::digraph_logic::types::{DigestedValuesResult, GraphModes, ObjectSelection, ParseError};
use crate::render::digraph_render::graph::{Graph};


#[function_component(DigraphPage)]
pub fn digraph_page() -> Html {
    // User determined state variables
    let input_value = use_state(String::new); // Used for controling user's input
    let graph_mode = use_state(|| GraphModes::DIGRAPH); // Tells us if we are rendering a digraph, matrix, or other
    let selection_object = use_state(|| ObjectSelection::default()); // Used for interactively selecting and highlighting elements

    // Calculated state variables
    let digested_values: UseStateHandle<DigestedValuesResult> = use_state(|| Err(ParseError::new("No input"))); // Parse user's input

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
        <TopbarLayout>
            <div class="app">
                // Inject our variables
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
        </TopbarLayout>
    }
}