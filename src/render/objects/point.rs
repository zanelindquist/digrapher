use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl Point {
    // Translate theta and distance params to x, y cordinates
    pub fn new(theta: f32, dist: f32) -> Self {
        Self { x: theta.cos() * dist, y: theta.sin() * dist }
    }

    pub fn distance_to(self, other: Point) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}