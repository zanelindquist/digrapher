use yew::prelude::*;
use web_sys::{HtmlElement};

use crate::{render::styles::{EquationStyle, MathErrorStyle, MatrixStyle, OperatorStyle, ScalarStyle}, services::{digraph_services::types::CanvasPositioning, matrix_services::{operators::OperatorPositioning, types::{MathErrorPositioning, MatrixEquation, ObjectSelection, Term}}, objects::{matrix::MatrixPositioning, scalar::ScalarPositioning}}};
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

    let equation_style = EquationStyle::default();

    let mut current_lx = -canvas_position.width as f32 / (3.0 * equation_style.vx_per_lx);
    let mut current_ly = 0.0;

    let rendered_terms = props.matrix_equation.terms.iter().map(|term| {
        // Matrices are centered a little differnt, so we have to do some math
        let vx = current_lx * equation_style.vx_per_lx;
        let vy = current_ly * equation_style.vx_per_lx;

        let width = match term {
            Term::Matrix(matrix) => matrix.width() * equation_style.matrix_style.cell_size as f32 / equation_style.vx_per_lx,
            Term::Scalar(scalar) => scalar.width(),
            Term::Operator(operator) => operator.width(),
            Term::Error(error) => 1.0
        };

        current_lx += width + equation_style.horizontal_spacing_lx;
        
        html! {
            {match term {
                Term::Matrix(matrix) => matrix.clone().draw(
                    &equation_style.matrix_style,
                    &MatrixPositioning::from_xy(vx + matrix.width() / 2.0 * equation_style.matrix_style.cell_size as f32, vy - equation_style.vx_per_lx / 2.0),
                    &canvas_position,
                    &crate::services::digraph_services::types::ObjectSelection::default()
                ),
                Term::Scalar(scalar) => scalar.draw(
                    &equation_style.scalar_style,
                    &ScalarPositioning::from_xy(vx, vy),
                    &canvas_position
                ),
                Term::Operator(operator) => operator.draw(
                    &equation_style.operator_style,
                    &OperatorPositioning::from_xy(vx, vy - operator.height() * equation_style.vx_per_lx),
                    &canvas_position
                ),
                Term::Error(error) => error.draw(
                    &equation_style.error_style,
                    &MathErrorPositioning::from_xy(vx, vy),
                    &canvas_position
                )
            }}
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
                width={canvas_position.width.to_string()}
                height={canvas_position.height.to_string()}
            >
            {for rendered_terms}
            </svg>
        </div>
    }
}