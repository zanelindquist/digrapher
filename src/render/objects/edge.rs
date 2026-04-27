use serde::{Serialize, Deserialize};

use super::point::Point;
use crate::logic::types::{PointRenderSymbol, RelationProperty};


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
            bearing: 0.0,
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
}