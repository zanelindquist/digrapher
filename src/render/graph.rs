use gloo_console::log;
use yew::prelude::*;
use crate::logic::digest_values::{digest_values};
use crate::logic::types::Relation;
use crate::render::canvas::{Canvas, CanvasPositioning};

#[derive(Properties, PartialEq)]
pub struct GraphProps {
    pub input: String,
}

#[function_component(Graph)]
pub fn graph(props: &GraphProps) -> Html{
    // Define our view settings
    let canvas_position = use_state(|| CanvasPositioning::new());

    let digested_values = digest_values(props.input.clone());

    match digested_values {
        Ok(relation) => html! {
            <div class="graph">
                <p>{format!("{:?}", relation.values)}</p>
                <Canvas position={(*canvas_position).clone()} relation={relation}/>
            </div>
        },
        Err(e) => html! {
            <div class="graph">
                <p>{ format!("Parsing error:\n{}", e.message)}</p>
            </div>
        }
    }
}