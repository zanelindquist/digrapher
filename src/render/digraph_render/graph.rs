use gloo_console::log;
use yew::prelude::*;
use web_sys::{HtmlElement};

use crate::services::digraph_services::types::{CanvasPositioning, DigestedValuesResult, GraphModes, ObjectSelection};
use crate::render::digraph_render::digraph_canvas::DigraphCanvas;
use crate::components::misc::toggle::{Toggle, ToggleOption};
use crate::render::digraph_render::matrix_canvas::MatrixCanvas;


#[derive(Properties, PartialEq)]
pub struct GraphProps {
    pub digested_values: UseStateHandle<DigestedValuesResult>,
    pub mode_change_callback: Callback<i32>,
    pub graph_mode: UseStateHandle<GraphModes>,
    pub object_selection: UseStateHandle<ObjectSelection>, // For just passing down the object selection information to child components at this time
}

#[function_component(Graph)]
pub fn graph(props: &GraphProps) -> Html{
    let node_ref = use_node_ref();
    let size = use_state(|| (1000, 1000));

    // Define our view settings
    let canvas_position = use_state(|| CanvasPositioning::new());

    let pointer_down = use_state(|| false);
    let last_pos = use_state(|| (0,0));
    let interrupt_scrolling = use_state(|| false);

    // On layout set the dimentions of the canvas
    use_effect({
        let node_ref = node_ref.clone();
        let size = size.clone();
        let canvas_pos = canvas_position.clone();

        move || {
            if let Some(element) = node_ref.cast::<HtmlElement>() {
                let width = element.offset_width();
                let height = element.offset_height();

                let rect = element.get_bounding_client_rect();
                let real_x = rect.x() as f32;
                let real_y = rect.y() as f32;

                let mut new_pos = (*canvas_pos).clone();
                // Set the canvas dom element's screen pixel coordinates
                new_pos.dom_element_offset_x = real_x;
                new_pos.dom_element_offset_y = real_y;

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

    // Support moving and zooming
    // When the pointer is clicked down
    let on_pointer_down = {
        let pointer_down = pointer_down.clone();
        let last_pos = last_pos.clone();
        let interrupt_scrolling = interrupt_scrolling.clone();
        Callback::from(move |e: PointerEvent| {
            if *interrupt_scrolling {
                return;
            }
            // Set the pointer status to down
            pointer_down.set(true);
            // Set the last_position as the mouse's current position
            last_pos.set((e.client_x(), e.client_y()));
        })
    };

    // When the pointer is lifed up
    let on_pointer_up = {
        let pointer_down = pointer_down.clone();
        Callback::from(move |_| {
            // Set the pointer to up
            pointer_down.set(false);
        })
    };

    // When the pointer moves around, we want to change the canvas_pos's x and y
    let on_pointer_move = {
        let pointer_down = pointer_down.clone();
        let last_pos = last_pos.clone();
        let canvas_position = canvas_position.clone();
        let interrupt_scrolling = interrupt_scrolling.clone();

        Callback::from(move |e: PointerEvent| {
            // If the pointer isn't down or the scrolling is interrupted, we aren't clicking and dragging the canvas
            if !*pointer_down || *interrupt_scrolling {
                return;
            }
            // Calculate the new canvas offset based on its current position and the mouse's position since the last move
            let new_offset_x = canvas_position.offset_x + e.x() - last_pos.0;
            let new_offset_y = canvas_position.offset_y + e.y() - last_pos.1;

            // Record the position of the mouse
            last_pos.set((e.x(), e.y()));

            // Update the canvas's position
            let mut updated_canvas_pos = (*canvas_position).clone();
            updated_canvas_pos.offset_x = new_offset_x;
            updated_canvas_pos.offset_y = new_offset_y;
            canvas_position.set(updated_canvas_pos);
        })
    };

    let on_wheel = {
        let canvas_position = canvas_position.clone();
        
        Callback::from(move |e: web_sys::WheelEvent| {
            let delta_y = e.delta_y();
            let new_zoom= (canvas_position.zoom + (delta_y / 1500.0) as f32).clamp(0.25, 5.0);

            let mut updated_canvas = (*canvas_position).clone();
            updated_canvas.zoom = new_zoom;

            canvas_position.set(updated_canvas);
        })
    };

    let toggle = html! {
        <Toggle
            class="graph__toggle"
            onchange={props.mode_change_callback.clone()}
        >
            <ToggleOption icon="digraph"/>
            <ToggleOption icon="matrix"/>
        </Toggle>
    };
    let graph_info = html! {
        <code class="graph__info">{format!(
            "{}x{} {:.1},{:.1} zoom: {:.2} {}",
            canvas_position.width, canvas_position.height,
            canvas_position.offset_x as f32, canvas_position.offset_y as f32,
            canvas_position.zoom,
            if *props.graph_mode == GraphModes::DIGRAPH {"digraph"} else {"matrix"}
        )}</code>
    };

    match &*props.digested_values {
        Ok(relation) => html! {
            <div
                ref={node_ref}
                class="graph"
                onpointerdown={on_pointer_down}
                onpointermove={on_pointer_move}
                onpointerup={on_pointer_up}
                onwheel={on_wheel}
            >
                {toggle}
                {graph_info}
                {
                    match *props.graph_mode {
                        GraphModes::DIGRAPH => html!{
                            <DigraphCanvas
                                class={if *pointer_down { "grab"} else {""}}
                                position={(*canvas_position).clone()}
                                relation={relation.clone()}
                                object_selection={props.object_selection.clone()}
                                interrupt_graph_scrolling={interrupt_scrolling}
                            />
                        },
                        GraphModes::MATRIX => html! {
                            <MatrixCanvas
                                position={(*canvas_position).clone()}
                                relation={relation.clone()}
                                object_selection={props.object_selection.clone()}
                            />
                        },
                        _ => html! {
                            <p>{"Unknown graph type"}</p>
                        }
                    }
                }

            </div>
        },
        Err(e) => html! {
            <div class="graph">
                {toggle}
                <code class="graph__error">{ format!("Parsing error: {}", e.message)}</code>
            </div>
        }
    }
}