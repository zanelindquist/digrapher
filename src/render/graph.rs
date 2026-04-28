use yew::prelude::*;
use web_sys::{HtmlElement};

use crate::logic::types::DigestedValuesResult;
use crate::render::canvas::{Canvas, CanvasPositioning};

#[derive(Properties, PartialEq)]
pub struct GraphProps {
    pub digested_values: UseStateHandle<DigestedValuesResult>
}

#[function_component(Graph)]
pub fn graph(props: &GraphProps) -> Html{
    let node_ref = use_node_ref();
    let size = use_state(|| (1000, 1000));

    // Define our view settings
    let canvas_position = use_state(|| CanvasPositioning::new());

    use_effect({
        let node_ref = node_ref.clone();
        let size = size.clone();
        let canvas_pos = canvas_position.clone();

        move || {
            if let Some(element) = node_ref.cast::<HtmlElement>() {
                let width = element.offset_width();
                let height = element.offset_height();
                web_sys::console::log_1(&format!("Size: {}x{}", width, height).into());

                let mut new_pos = (*canvas_pos).clone();

                if new_pos.width != width || new_pos.height != height {
                    new_pos.width = width;
                    new_pos.height = height;

                    canvas_pos.set(new_pos);
                    size.set((width, height))
                }
            }
            || ()
        }
    });


    match &*props.digested_values {
        Ok(relation) => html! {
            <div
                ref={node_ref}
                class="graph"
            >
                // <p>{format!("{:?}", relation.values)}</p>
                <Canvas
                    position={(*canvas_position).clone()}
                    relation={relation.clone()}
                />
            </div>
        },
        Err(e) => html! {
            <div class="graph">
                <p class="graph__error">{ format!("Parsing error: {}", e.message)}</p>
            </div>
        }
    }
}