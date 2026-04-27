use std::f32::consts::PI;

use crate::logic::types::{PointVector, RawCharPoints};
use crate::render::canvas::CanvasPositioning;
use crate::render::objects::point::{Point};
use crate::render::objects::edge::{Edge};


pub fn position_points(points: RawCharPoints, position: CanvasPositioning) -> PointVector {
    let n = points.len();
    let mut point_vec = PointVector::new();
    let r = (position.width as f32) / 3.0 * position.zoom;
    let center_x = (position.width as f32) / 2.0 + position.offset_x as f32;
    let center_y = (position.height as f32) / 2.0 + position.offset_y as f32;

    web_sys::console::log_1(&format!("Center: {}x{}", center_x, center_y).into());


    for (i, p) in points.iter().enumerate() {
        let theta = (i as f32) * (2.0 * PI / n as f32);
        let x = center_x + r * theta.cos();
        let y = center_y + r * theta.sin();
        let new_point = Point::new(x, y, p.clone());
        point_vec.push(new_point);
    }

    point_vec
}