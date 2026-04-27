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
    let points: PointVector = position_points(props.relation.points.clone(), props.position);
    let edges: EdgeVector = position_edges(props.relation.values.clone(), points.clone());
    let styles = props.styles;

    html!{
        <div class="canvas" >
            <svg
                class="canvas__svg"
                width={props.position.width.to_string()}
                height={props.position.height.to_string()}
            >
                { for edges.iter().map(|edge| {
                    match edge.relation_type {
                        RelationProperty::REFLEXIVE => html!{

                        },
                        RelationProperty::SYMMETRIC => html! {

                        },
                        // Antisymmetrics
                        _=> html! {
                            <>
                                <line
                                    x1={edge.start.x.to_string()}
                                    y1={edge.start.y.to_string()}
                                    x2={edge.end.x.to_string()}
                                    y2={edge.end.y.to_string()}
                                    stroke={styles.edge.stroke.to_string()}
                                    stroke-width={styles.edge.stroke_width.to_string()}
                                />
                                // Render the midpoint
                                
                            </>
                        }
                    }
                })}

                { for points.iter().map(|point| html! {
                    <>
                        <circle
                            cx={point.x.to_string()}
                            cy={point.y.to_string()}
                            r={styles.dot.radius.to_string()}
                            fill={styles.dot.fill}
                            stroke={styles.dot.stroke}
                            stroke-width={styles.dot.stroke_width.to_string()}
                        />
                        <text
                            x={(point.x + point.bearing.cos() * styles.font.size).to_string()}
                            y={(point.y + point.bearing.sin() * styles.font.size).to_string()}
                            font-family={styles.font.family}
                            font-size={styles.font.size.to_string()}
                            fill={styles.font.fill}
                        >
                            { point.label.to_string() }
                        </text>
                    </>
                })}
            </svg>
        </div>
    }
}