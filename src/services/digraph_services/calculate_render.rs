use std::f32::consts::PI;

use crate::render::objects::edge::Edge;
use crate::render::objects::point::Point;
use crate::services::digraph_services::types::{
    EdgeVector, PointRenderSymbol, PointVector, RawEdgePairs, RelationProperty, SortedCharPoints,
};

// Outputs points with logical coordinates in -1 to 1 x, y plane
pub fn position_points(points: &SortedCharPoints) -> PointVector {
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

pub fn position_points_preserving_existing(
    points: &SortedCharPoints,
    existing_points: &PointVector,
) -> PointVector {
    position_points(points)
        .into_iter()
        .map(|mut point| {
            if let Some(existing_point) = existing_points
                .iter()
                .find(|existing_point| existing_point.label == point.label)
            {
                point.x = existing_point.x;
                point.y = existing_point.y;
            }
            point
        })
        .collect()
}

// Transforms the raw text edge parings (a, b) into edges that correspond to real points
pub fn create_edges(values: &RawEdgePairs, points: &PointVector) -> EdgeVector {
    let mut edges = EdgeVector::new();

    for (_, pair) in values.iter().enumerate() {
        // Every pair will produce an edge
        let point1 = points.iter().find(|p| p.label == pair.0).unwrap();
        let point2 = points.iter().find(|p| p.label == pair.1).unwrap();
        let relation_type = if pair.0 == pair.1 {
            RelationProperty::REFLEXIVE
        } else if values
            .iter()
            .find(|v| v.0 == pair.1 && v.1 == pair.0)
            .is_some()
        {
            RelationProperty::SYMMETRIC
        } else {
            RelationProperty::ANTISYMMETRIC
        };

        edges.push(Edge::new(
            (*point1).clone(),
            (*point2).clone(),
            relation_type,
        ));
    }

    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_points_preserving_existing_keeps_custom_coordinates_by_label() {
        let labels = vec![String::from("A"), String::from("B"), String::from("C")];
        let existing_points = vec![
            Point::new(
                0.25,
                -0.75,
                0.0,
                String::from("C"),
                PointRenderSymbol::CIRCLE,
                1,
            ),
            Point::new(
                -0.5,
                0.5,
                0.0,
                String::from("A"),
                PointRenderSymbol::CIRCLE,
                0,
            ),
        ];

        let points = position_points_preserving_existing(&labels, &existing_points);

        let point_a = points.iter().find(|point| point.label == "A").unwrap();
        let point_c = points.iter().find(|point| point.label == "C").unwrap();
        assert_eq!((point_a.x, point_a.y), (-0.5, 0.5));
        assert_eq!((point_c.x, point_c.y), (0.25, -0.75));
        assert_eq!(point_c.index, 2);
    }
}
