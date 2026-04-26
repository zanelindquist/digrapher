use super::point::Point;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
    pub directed: bool,
}

impl Edge {
    pub fn new(start: Point, end: Point, directed: bool) -> Self {
        Self { start, end, directed }
    }
    
    pub fn midpoint(self) -> Point {
        Point {
            x: (self.start.x + self.end.x) / 2.0,
            y: (self.start.y + self.end.y) / 2.0,
            label: String::from("")
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