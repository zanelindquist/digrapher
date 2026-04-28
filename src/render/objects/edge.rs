use std::f32::consts::PI;

use js_sys::Math::atan2;
use yew::prelude::*;
use serde::{Serialize, Deserialize};

use super::point::Point;
use crate::{logic::types::{PointRenderSymbol, RelationProperty}, render::styles::RenderStyles};


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
    pub relation_type: RelationProperty,
    pub bezier_arc_px: f32
}

impl Edge {
    pub fn new(start: Point, end: Point, relation_type: RelationProperty) -> Self {
        Self { start, end, relation_type, bezier_arc_px: 40.0 }
    }
    
    pub fn midpoint(self) -> Point {
        let x = (self.start.x + self.end.x) / 2.0 + (self.clone().angle() + PI / 2.0).cos() * self.bezier_arc_px;
        let y = (self.start.y + self.end.y) / 2.0 + (self.clone().angle() + PI / 2.0).sin() * self.bezier_arc_px;
        
        match self.relation_type {
            RelationProperty::SYMMETRIC => Point {
                // Point needs to be displaced 90 degrees orthagonal to the bearing
                x, y,
                bearing: self.angle(),
                label: String::from(""),
                symbol: PointRenderSymbol::TRIANGLE
            },
            // Antisymmetric default
            _=> Point {
                x: (self.start.x + self.end.x) / 2.0,
                y: (self.start.y + self.end.y) / 2.0,
                bearing: self.angle(),
                label: String::from(""),
                symbol: PointRenderSymbol::TRIANGLE
            }
        }
    }

    pub fn bezier_control_point(self) -> Point {
        let pixel_displacement = self.bezier_arc_px * 2.0;
        let x = (self.start.x + self.end.x) / 2.0 + (self.clone().angle() + PI / 2.0).cos() * pixel_displacement;
        let y = (self.start.y + self.end.y) / 2.0 + (self.clone().angle() + PI / 2.0).sin() * pixel_displacement;
        
        Point {
            x, y,
            bearing: self.angle(),
            label: String::from(""),
            symbol: PointRenderSymbol::TRIANGLE
        }
    }
    
    pub fn length(self) -> f32 {
        self.start.distance_to(self.end)
    }
    
    // Calculate angle for arrow head
    pub fn angle(self) -> f32 {
        (self.end.y - self.start.y).atan2(self.end.x - self.start.x)
    }

    pub fn draw(self, styles: RenderStyles) -> Html {
        match self.relation_type {
            RelationProperty::REFLEXIVE => html!{

            },
            RelationProperty::SYMMETRIC => html! {

                <>
                    <path
                        d={format!("M {} Q {} {}", 
                            // Move to point
                            format!("{} {}", self.end.x, self.end.y),
                            // Quadradic belzier curve
                            format!("{} {}",
                                self.clone().bezier_control_point().x.to_string(),
                                self.clone().bezier_control_point().y.to_string()
                            ),
                            format!("{} {}", self.start.x, self.start.y),
                            
                        )}
                        fill={"transparent"}
                        stroke={styles.edge.stroke.to_string()}
                        stroke-width={styles.edge.stroke_width.to_string()}
                    />
                    // Render the midpoint
                    {self.midpoint().draw(styles)}
                </>
            },
            // Antisymmetrics
            _=> html! {
                <>
                    <line
                        x1={self.start.x.to_string()}
                        y1={self.start.y.to_string()}
                        x2={self.end.x.to_string()}
                        y2={self.end.y.to_string()}
                        stroke={styles.edge.stroke.to_string()}
                        stroke-width={styles.edge.stroke_width.to_string()}
                    />
                    // Render the midpoint
                    {self.midpoint().draw(styles)}
                </>
            }
        }
    }
}