use yew::prelude::*;

use crate::logic::calculate_render::position_points;
use crate::logic::types::{PointVector, Relation};
use crate::render::objects::point::{Point};
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
            width: 1000,
            height: 1000,
            zoom: 1.0
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CanvasProps {
    pub position: CanvasPositioning,
    pub relation: Relation,
    #[prop_or_default]
    pub styles: RenderStyles
}

#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {

    let points: PointVector = position_points(props.relation.points.clone(), props.position);
    let styles = props.styles;
    html!{
        <svg width={props.position.width.to_string()} height={props.position.height.to_string()} style="canvas">
            { for points.iter().map(|point| html! {
                <circle
                    cx={point.x.to_string()}
                    cy={point.y.to_string()}
                    r={styles.dot.radius.to_string()}
                    fill={styles.dot.fill}
                    stroke={styles.dot.stroke}
                    stroke-width={styles.dot.stroke_width.to_string()}
                />
            })}
        </svg>
    }
}