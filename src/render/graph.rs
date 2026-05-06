use gloo_console::log;
use yew::prelude::*;
use web_sys::{HtmlElement};

use crate::logic::types::{CanvasPositioning, DigestedValuesResult, GraphModes, ObjectSelection};
use crate::render::digraph_canvas::DigraphCanvas;
use crate::components::toggle::{Toggle, ToggleOption};
use crate::render::matrix_canvas::MatrixCanvas;


#[derive(Properties, PartialEq)]
pub struct GraphProps {
    pub digested_values: UseStateHandle<DigestedValuesResult>,
    pub mode_change_callback: Callback<i32>,
    pub graph_mode: UseStateHandle<GraphModes>,
    pub object_selection: UseStateHandle<ObjectSelection>
}

#[function_component(Graph)]
pub fn graph(props: &GraphProps) -> Html{
    let node_ref = use_node_ref();
    let size = use_state(|| (1000, 1000));

    // Define our view settings
    let canvas_position = use_state(|| CanvasPositioning::new());

    let pointer_down = use_state(|| false);
    let last_pos = use_state(|| (0,0));

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

    // Support moving and zooming
    let on_pointer_down = {
        let pointer_down = pointer_down.clone();
        let last_pos = last_pos.clone();
        Callback::from(move |e: PointerEvent| {
            pointer_down.set(true);
            last_pos.set((e.client_x(), e.client_y()));
        })
    };

    let on_pointer_up = {
        let pointer_down = pointer_down.clone();
        Callback::from(move |_| {
            pointer_down.set(false);
        })
    };

    let on_pointer_move = {
        let pointer_down = pointer_down.clone();
        let last_pos = last_pos.clone();
        let canvas_position = canvas_position.clone();

        Callback::from(move |e: web_sys::PointerEvent| {
            if !*pointer_down {
                return;
            }
            let new_offset_x = canvas_position.offset_x + e.x() - last_pos.0;
            let new_offset_y = canvas_position.offset_y + e.y() - last_pos.1;

            last_pos.set((e.x(), e.y()));

            canvas_position.set(CanvasPositioning::create(new_offset_x, new_offset_y, canvas_position.width, canvas_position.height, canvas_position.zoom))
        })
    };

    let on_wheel = {
        let canvas_position = canvas_position.clone();
        
        Callback::from(move |e: web_sys::WheelEvent| {
            let delta_y = e.delta_y();
            let new_zoom= (canvas_position.zoom + (delta_y / 1500.0) as f32).clamp(0.25, 5.0);

            canvas_position.set(CanvasPositioning::create(canvas_position.offset_x, canvas_position.offset_y, canvas_position.width, canvas_position.height,
                new_zoom
            ));
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