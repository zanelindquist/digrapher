use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub label: String,
}

impl Point {
    pub fn new(x: f32, y: f32, label: String) -> Self {
        Self { x, y, label }
    }
    
    pub fn distance_to(self, other: Point) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

}