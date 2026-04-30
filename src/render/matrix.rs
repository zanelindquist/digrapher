use yew::prelude::*;

use crate::logic::calculate_render::{position_edges, position_points};
use crate::logic::types::{CanvasPositioning, EdgeVector, Matrix, PointVector, Relation};
use crate::render::styles::RenderStyles;


#[derive(Properties, PartialEq)]
pub struct MatrixProps {
    pub position: CanvasPositioning,
    pub relation: Relation,
    #[prop_or_default]
    pub styles: RenderStyles,
    #[prop_or_default]
    pub class: Classes
}

#[function_component(MatrixGraph)]
pub fn matrix(props: &MatrixProps) -> Html {
    // Sort the points first so we have a same order every time
    let mut sorted_points: Vec<String> = props.relation.points.clone().into_iter().collect();
    sorted_points.sort();

    let points: PointVector = position_points(sorted_points, props.position);
    let edges: EdgeVector = position_edges(props.relation.values.clone(), points.clone());
    let styles = props.styles;

    let matrix: Matrix = Matrix::from_edges(points, edges);

    html!{
        <div
            class={classes!("matrix", props.class.clone())}
        >
            <svg
                class="matrix__svg"
                width={props.position.width.to_string()}
                height={props.position.height.to_string()}
            >
                {matrix.draw(styles, props.position)}
            </svg>
        </div>
    }
}