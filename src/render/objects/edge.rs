use std::f32::consts::{PI, SQRT_2};

use yew::prelude::*;
use serde::{Serialize, Deserialize};

use super::point::Point;
use crate::{render::styles::RenderStyles, services::digraph_services::types::{CanvasPositioning, PointRenderSymbol, RelationProperty}};


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
    pub relation_type: RelationProperty,
    pub bezier_arc_px: f32,
    pub loop_radius: f32
}

impl Edge {
    pub fn new(start: Point, end: Point, relation_type: RelationProperty) -> Self {
        Self { start, end, relation_type, bezier_arc_px: 40.0, loop_radius: 40.0 }
    }
    
    pub fn midpoint(&self) -> Point {
        // Midpoint of for a symetric line
        let sym_x = (self.start.x + self.end.x) / 2.0 + (self.angle() + PI / 2.0).cos() * self.bezier_arc_px;
        let sym_y = (self.start.y + self.end.y) / 2.0 + (self.angle() + PI / 2.0).sin() * self.bezier_arc_px;

        // Midpoint for a reflexive line
        let ref_x = self.start.x + (self.start.bearing).cos() * (self.loop_radius * (1.0 + SQRT_2));
        let ref_y = self.start.y + (self.start.bearing).sin() * (self.loop_radius * (1.0  + SQRT_2));
        
        match self.relation_type {
            RelationProperty::SYMMETRIC => Point {
                // Point needs to be displaced 90 degrees orthagonal to the bearing
                x: sym_x, y: sym_y,
                bearing: self.angle(),
                label: String::from(""),
                symbol: PointRenderSymbol::TRIANGLE,
                index: 0
            },
            RelationProperty::REFLEXIVE => Point {
                x: ref_x, y: ref_y,
                bearing: self.start.bearing + PI / 2.0,
                label: String::from(""),
                symbol: PointRenderSymbol::TRIANGLE,
                index: 0
            },
            // Antisymmetric default
            _=> Point {
                x: (self.start.x + self.end.x) / 2.0,
                y: (self.start.y + self.end.y) / 2.0,
                bearing: self.angle(),
                label: String::from(""),
                symbol: PointRenderSymbol::TRIANGLE,
                index: 0
            }
        }
    }

    pub fn bezier_control_point(&self,canvas_pos: &CanvasPositioning) -> Point {
        let arc_displacement = canvas_pos.visual_to_logical_scalar(self.bezier_arc_px * 2.0);
        let x = (self.start.x + self.end.x) / 2.0 + (self.angle() + PI / 2.0).cos() * arc_displacement;
        let y = (self.start.y + self.end.y) / 2.0 + (self.angle() + PI / 2.0).sin() * arc_displacement;
        
        Point {
            x, y,
            bearing: self.angle(),
            label: String::from(""),
            symbol: PointRenderSymbol::TRIANGLE,
            index: 0
        }
    }
    
    pub fn length(self) -> f32 {
        self.start.distance_to(self.end.x, self.end.y)
    }
    
    // Calculate angle for arrow head
    pub fn angle(&self) -> f32 {
        (self.end.y - self.start.y).atan2(self.end.x - self.start.x)
    }

    pub fn draw(self, styles: &RenderStyles, canvas_pos: &CanvasPositioning, is_selected: bool) -> Html {
        // Convert from logical to visual pixels
        let (start_x, start_y) = canvas_pos.logical_to_visual_xy(self.start.x, self.start.y);
        let (end_x, end_y) = canvas_pos.logical_to_visual_xy(self.end.x, self.end.y);
        let bez = self.bezier_control_point(&canvas_pos);
        let (bez_x, bez_y) = canvas_pos.logical_to_visual_xy(bez.x, bez.y);

        let stroke_color = if is_selected {styles.edge.highlighted_stroke.to_string() } else { styles.edge.stroke.to_string()};

        match self.relation_type {
            RelationProperty::REFLEXIVE => html!{
                <>
                    <path
                        d={format!(
                            "M {}
                            L {}
                            A {}
                            Z",
                            // Move to
                            format!("{} {}", end_x, end_y),
                            format!("{} {}",
                                start_x + (self.start.bearing - PI / 4.0).cos() * self.loop_radius,
                                start_y + (self.start.bearing - PI / 4.0).sin() * self.loop_radius,
                            ),
                            format!("{} {}, 0, 1, 1, {} {}",
                                self.loop_radius,
                                self.loop_radius, 
                                // Start arc point
                                start_x + (self.start.bearing + PI / 4.0).cos() * self.loop_radius,
                                start_y + (self.start.bearing + PI / 4.0).sin() * self.loop_radius,
                            )
                        )}
                        fill="transparent"
                        stroke={stroke_color}
                        stroke-width={styles.edge.stroke_width.to_string()}
                    />
                    {self.midpoint().draw(styles, canvas_pos, is_selected)}
                </>
            },
            RelationProperty::SYMMETRIC => html! {

                <>
                    <path
                        d={format!("M {} Q {} {}", 
                            // Move to point
                            format!("{} {}", end_x, end_y),
                            // Quadradic belzier curve
                            format!("{} {}",
                                bez_x.to_string(),
                                bez_y.to_string()
                            ),
                            format!("{} {}", start_x, start_y),
                            
                        )}
                        fill={"transparent"}
                        stroke={stroke_color}
                        stroke-width={styles.edge.stroke_width.to_string()}
                    />
                    // Render the midpoint
                    {self.midpoint().draw(styles, canvas_pos, is_selected)}
                </>
            },
            // Antisymmetrics
            _=> html! {
                <>
                    <line
                        x1={start_x.to_string()}
                        y1={start_y.to_string()}
                        x2={end_x.to_string()}
                        y2={end_y.to_string()}
                        stroke={stroke_color}
                        stroke-width={styles.edge.stroke_width.to_string()}
                    />
                    // Render the midpoint
                    {self.midpoint().draw(styles, canvas_pos, is_selected)}
                </>
            }
        }
    }
}