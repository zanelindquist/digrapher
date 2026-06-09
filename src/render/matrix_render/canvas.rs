use yew::prelude::*;
use web_sys::{HtmlElement};

use crate::{render::styles::MatrixStyle, services::{digraph_services::types::CanvasPositioning, matrix_services::types::{MatrixEquation, ObjectSelection, Term}, objects::matrix::MatrixPositioning}};
use crate::services::objects::{matrix::Matrix, scalar::Scalar};


#[derive(Properties, PartialEq)]
pub struct MatrixCanvasProps {
    pub matrix_equation: MatrixEquation,
    pub object_selection: UseStateHandle<ObjectSelection>,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(MatrixCanvas)]
pub fn matrix_canvas(props: &MatrixCanvasProps) -> Html {
    let node_ref = use_node_ref();
    let size = use_state(|| (1000, 1000));

    let canvas_position = use_state(|| CanvasPositioning::new());

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

    html! {
        <div
            ref={node_ref}
            class={classes!("canvas", props.class.clone())}
            // style={format!("cursor: {};", cursor_pointer)}
        >
            <svg
                class="canvas__svg"
            >
            {for props.matrix_equation.terms.iter().enumerate().map(|(index, term)| {
                html! {
                    {match term {
                        Term::Matrix(matrix) => html! {
                            matrix.clone().draw(
                                &MatrixStyle::default(),
                                &MatrixPositioning::from_xy(index as f32 * 200.0, 0.0),
                                &canvas_position,
                                &crate::services::digraph_services::types::ObjectSelection::default()
                            )
                        },
                        Term::Scalar(scalar) => html! {

                        }
                    }}
                }
            })}
            </svg>
        </div>
    }
}