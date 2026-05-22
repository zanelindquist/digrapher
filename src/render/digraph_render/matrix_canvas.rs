use yew::prelude::*;

use crate::services::digraph_services::point_layout::{create_edges, create_points};
use crate::services::digraph_services::types::{CanvasPositioning, EdgeVector, ObjectSelection, PointVector, Relation};
use crate::services::objects::matrix::{Matrix};
use crate::render::objects::point::Point;
use crate::render::styles::RenderStyles;


#[derive(Properties, PartialEq)]
pub struct MatrixCanvasProps {
    pub position: CanvasPositioning,
    pub relation: Relation,
    #[prop_or_default]
    pub styles: RenderStyles,
    #[prop_or_default]
    pub class: Classes,
    pub object_selection: UseStateHandle<ObjectSelection>
}

#[function_component(MatrixCanvas)]
pub fn matrix_canvas(props: &MatrixCanvasProps) -> Html {
    // Sort the points first so we have a same order every time
    let mut sorted_points: Vec<String> = props.relation.points.clone().into_iter().collect();
    sorted_points.sort();

    let points: PointVector = create_points(&sorted_points);
    let edges: EdgeVector = create_edges(&props.relation.values, &points);
    let styles = props.styles;

    let matrix: Matrix = Matrix::from_edges(points, edges);
    let center_point: Point = Point::from_xy((props.position.width/2) as f32, (props.position.height/2) as f32);

    html!{
        <div
            class={classes!("matrix", props.class.clone())}
        >
            <svg
                class="matrix__svg"
                width={props.position.width.to_string()}
                height={props.position.height.to_string()}
            >
                {matrix.draw(&styles.matrix, &center_point, props.object_selection.clone())}
            </svg>
        </div>
    }
}