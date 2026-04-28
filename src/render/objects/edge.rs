use yew::prelude::*;
use serde::{Serialize, Deserialize};

use super::point::Point;
use crate::{logic::types::{PointRenderSymbol, RelationProperty}, render::styles::RenderStyles};


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
    pub relation_type: RelationProperty,
}

impl Edge {
    pub fn new(start: Point, end: Point, relation_type: RelationProperty) -> Self {
        Self { start, end, relation_type }
    }
    
    pub fn midpoint(self) -> Point {
        Point {
            x: (self.start.x + self.end.x) / 2.0,
            y: (self.start.y + self.end.y) / 2.0,
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