use std::f32::consts::PI;

use crate::logic::types::{EdgeVector, PointRenderSymbol, PointVector, RawCharPoints, RawEdgePairs, RelationProperty};
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
        web_sys::console::log_1(&format!("Pos {}: {}x{}", i, x, y).into());
        let new_point = Point::new(x, y, theta, p.clone(), PointRenderSymbol::CIRCLE);
        point_vec.push(new_point);
    }

    point_vec
}

pub fn position_edges(values: RawEdgePairs, points: PointVector) -> EdgeVector {
    let mut edges = EdgeVector::new();

    for (i, pair) in values.iter().enumerate() {
        // Every pair will produce an edge
        let point1 = points.iter().find(|p| p.label == pair.0 ).unwrap();
        let point2 = points.iter().find(|p| p.label == pair.1 ).unwrap();
        let relation_type = if pair.0 == pair.1 {
            RelationProperty::REFLEXIVE
        } else if values.iter().find(|v| v.0 == pair.1 && v.1 == pair.0).is_some() {
            RelationProperty::REFLEXIVE
        } else {
            RelationProperty::ANTISYMMETRIC
        };

        edges.push(Edge::new((*point1).clone(), (*point2).clone(), relation_type));
    }

    edges
}