use std::ops::Deref;

use yew::prelude::*;

use crate::logic::calculate_render::{position_edges, position_points};
use crate::logic::types::{EdgeVector, PointVector, Relation, RelationProperty};
use crate::render::styles::RenderStyles;

#[derive(Clone, Copy, PartialEq)]
pub struct CanvasPositioning {
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: i32,
    pub height: i32,
    pub zoom: f32,
}
impl CanvasPositioning {
    pub fn new() -> Self {
        Self {
            offset_x: 0,
            offset_y: 0,
            width: 300,
            height: 300,
            zoom: 1.0,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CanvasProps {
    pub position: CanvasPositioning,
    pub relation: Relation,
    #[prop_or_default]
    pub styles: RenderStyles,
}

#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {
    // Sort the points first so we have a same order every time
    let mut sorted_points: Vec<String> = props.relation.points.clone().into_iter().collect();
    sorted_points.sort();

    let points: PointVector = position_points(sorted_points, props.position);
    let edges: EdgeVector = position_edges(props.relation.values.clone(), points.clone());
    let styles = props.styles;

    html!{
        <div class="canvas" >
            <svg
                class="canvas__svg"
                width={props.position.width.to_string()}
                height={props.position.height.to_string()}
            >
                { for edges.iter().map(|edge| { edge.clone().draw(styles) })}
                { for points.iter().map(|point| { point.clone().draw(styles) })}
            </svg>
        </div>
    }
}