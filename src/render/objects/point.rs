use serde::{Serialize, Deserialize};

use crate::logic::types::PointRenderSymbol;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub bearing: f32, // Direction away from the centerpoint
    pub label: String,
    pub symbol: PointRenderSymbol
}

impl Point {
    pub fn new(x: f32, y: f32, bearing: f32, label: String, symbol: PointRenderSymbol) -> Self {
        Self { x, y, bearing, label, symbol }
    }
    
    pub fn distance_to(self, other: Point) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

}