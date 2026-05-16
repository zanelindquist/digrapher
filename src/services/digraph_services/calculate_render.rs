use std::cmp::min;
use std::f32::consts::PI;

use crate::services::digraph_services::types::{EdgeVector, PointRenderSymbol, PointVector, RawEdgePairs, RelationProperty, SortedCharPoints};
use crate::services::digraph_services::types::CanvasPositioning;
use crate::render::objects::point::{Point};
use crate::render::objects::edge::{Edge};

// Outputs points with logical coordinates in -1 to 1 x, y plane
pub fn position_points(points: SortedCharPoints) -> PointVector {
    let n = points.len();
    let mut point_vec = PointVector::new();

    for (i, p) in points.iter().enumerate() {
        // Draw counterclockwise
        let theta = -(i as f32) * (2.0 * PI / n as f32);
        let x = theta.cos();
        let y = theta.sin();
        let new_point = Point::new(x, y, theta, p.clone(), PointRenderSymbol::CIRCLE, i as i32);
        point_vec.push(new_point);
    }

    point_vec
}

pub fn position_edges(values: RawEdgePairs, points: PointVector) -> EdgeVector {
    let mut edges = EdgeVector::new();

    for (_, pair) in values.iter().enumerate() {
        // Every pair will produce an edge
        let point1 = points.iter().find(|p| p.label == pair.0 ).unwrap();
        let point2 = points.iter().find(|p| p.label == pair.1 ).unwrap();
        let relation_type = if pair.0 == pair.1 {
            RelationProperty::REFLEXIVE
        } else if values.iter().find(|v| v.0 == pair.1 && v.1 == pair.0).is_some() {
            RelationProperty::SYMMETRIC
        } else {
            RelationProperty::ANTISYMMETRIC
        };

        edges.push(Edge::new((*point1).clone(), (*point2).clone(), relation_type));
    }

    edges
}