use yew::prelude::*;
use crate::components::digraph_components::sidebar::Sidebar;
use crate::components::navigation::topbar_layout::TopbarLayout;
use crate::services::digraph_services::classify_relation::{process_reltaion};
use crate::services::digraph_services::digest_values::{digest_values};
use crate::services::digraph_services::types::{DigestedValuesResult, GraphEditCallbacks, GraphModes, GraphTooltips, ObjectSelection, ParseError, ProcessedRelationResult};
use crate::render::digraph_render::graph::{Graph};


#[function_component(DigraphPage)]
pub fn digraph_page() -> Html {
    // User determined state variables
    let input_value = use_state(String::new); // Used for controling user's input
    let graph_mode = use_state(|| GraphModes::DIGRAPH); // Tells us if we are rendering a digraph, matrix, or other
    let selection_object = use_state(|| ObjectSelection::default()); // Used for interactively selecting and highlighting elements
    let graph_editing_mode: UseStateHandle<Option<GraphTooltips>> = use_state(|| None);

    // Calculated state variables
    let digested_values: UseStateHandle<DigestedValuesResult> = use_state(|| Err(ParseError::new("No input"))); // Parse user's input
    let processed_relation: UseStateHandle<ProcessedRelationResult> = use_state(|| Err(ParseError::new("No input"))); // Parse user's input

    // When a user changes their input value
    let on_input = {
        let input_value = input_value.clone();
        let digested_values = digested_values.clone();
        let processed_relation = processed_relation.clone();
        Callback::from(move |v: String| {
            // Set the input value
            input_value.set(v.clone());
            let result = digest_values(v);
            digested_values.set(result.clone());
            // Update the digested values for our pipeline
            match result {
                Ok(dv) => {
                    processed_relation.set(process_reltaion(dv));
                },
                // Only set the error if there is no input, because we only care about that one
                Err(e) => {
                    if e.message == "No input" {
                        processed_relation.set(Err(e));
                    }
                }
            }
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

    // Point editing, creation and deletion
    let graph_edit_callbacks = GraphEditCallbacks{
        on_edit_point: {
            let processed_relation = processed_relation.clone();
            Callback::from(move |(label, lx, ly): (String, f32, f32)| {
                let gtrm = (*processed_relation).clone();
                match gtrm {
                    Ok(mut gm) => {
                        if let Ok(new_gm) = gm.edit_point(label.clone(), lx, ly) {
                            processed_relation.set(Ok(new_gm));
                        }
                        // HANDLE POINT SELECTION ERROR
                    },
                    Err(_) => {}
                }
            })
        },
        on_point_create: {
            let processed_relation = processed_relation.clone();
            // Expects logical units
            Callback::from(move |(label, lx, ly): (String, f32, f32)| {
                let gtrm = (*processed_relation).clone();
                match gtrm {
                    Ok(mut gm) => {
                        if let Ok(new_gm) = gm.create_point(label, lx, ly) {
                            processed_relation.set(Ok(new_gm));
                        }
                        // HANDLE POINT SELECTION ERROR
                    },
                    Err(_) => {}
                }
            })
        },
        on_point_delete: {
            let processed_relation = processed_relation.clone();
            Callback::from(move |label: String| {
                let gtrm = (*processed_relation).clone();
                match gtrm {
                    Ok(mut gm) => {
                        if let Ok(new_gm) = gm.delete_point(label.clone()) {
                            processed_relation.set(Ok(new_gm));
                        }
                        // HANDLE POINT SELECTION ERROR
                    },
                    Err(_) => {}
                }
            })
        },
        on_edge_connection: {
            let processed_relation = processed_relation.clone();
            // Expects logical units
            Callback::from(move |(from_label, to_label): (String, String)| {
                let gtrm = (*processed_relation).clone();
                match gtrm {
                    Ok(mut gm) => {
                        if let Ok(new_gm) = gm.connect_edge(from_label, to_label) {
                            processed_relation.set(Ok(new_gm));
                        }
                    },
                    Err(_) => {}
                }
            })
        },
        on_point_create_and_connect: {
            let processed_relation = processed_relation.clone();
            Callback::from(move |(label, lx, ly, from_label): (String, f32, f32, String)| {
                let gtrm = (*processed_relation).clone();
                match gtrm {
                    Ok(mut gm) => {
                        if let Ok(new_gm) = gm.create_point(label.clone(), lx, ly)
                            .and_then(|mut gm| gm.connect_edge(from_label, label)) {
                            processed_relation.set(Ok(new_gm));
                        }
                    },
                    Err(_) => {}
                }
            })
        }
    };

    // Set page name
    use_effect(|| {
        gloo_utils::document().set_title("Digraph");
        || ()
    });

    html! {
        <TopbarLayout>
            <div class="app">
                // Render the sidebar
                <Sidebar
                    processed_relation={processed_relation.clone()}
                    value={(*input_value).clone()}
                    on_input={on_input}
                    digested_values={digested_values.clone()}
                    object_selection={selection_object.clone()}
                    graph_editing_mode={graph_editing_mode.clone()}
                />
                // Render the graph
                <Graph
                    processed_relation={processed_relation.clone()}
                    mode_change_callback={on_mode_change}
                    graph_mode={graph_mode.clone()}
                    object_selection={selection_object}
                    graph_edit_callbacks={graph_edit_callbacks}
                    graph_editing_mode={graph_editing_mode.clone()}
                />
            </div>
        </TopbarLayout>
    }
}