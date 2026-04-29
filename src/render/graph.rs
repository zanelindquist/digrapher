use yew::prelude::*;
use web_sys::{HtmlElement};

use crate::logic::types::DigestedValuesResult;
use crate::render::canvas::{Canvas, CanvasPositioning};
use crate::components::toggle::{Toggle, ToggleOption};

#[derive(Properties, PartialEq)]
pub struct GraphProps {
    pub digested_values: UseStateHandle<DigestedValuesResult>,
    pub mode_change_callback: Callback<i32>
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

    let toggle = html! {
        <Toggle
            callback={props.mode_change_callback.clone()}
        >
            <ToggleOption icon="digraph"/>
            <ToggleOption icon="matrix"/>
        </Toggle>
    };

    match &*props.digested_values {
        Ok(relation) => html! {
            <div
                ref={node_ref}
                class="graph"
            >
                // <p>{format!("{:?}", relation.values)}</p>
                {toggle}
                <Canvas
                    position={(*canvas_position).clone()}
                    relation={relation.clone()}
                />
            </div>
        },
        Err(e) => html! {
            <div class="graph">
                {toggle}
                <p class="graph__error">{ format!("Parsing error: {}", e.message)}</p>
            </div>
        }
    }
}